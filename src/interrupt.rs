//! Demonstrates triggering and handling a software interrupt
//!
//! # Safety
//!
//! This sample uses two `static mut` items to facilitate direct access to
//! peripherals from both `main` and the global interrupt handler. The safety of
//! this approach relies on **never** taking an interrupt while the "main"
//! thread is accessing a peripheral.
//!
//! While this safety constraint is easy to observe when using only synchronous
//! software interrupts (as in this example), true hardware interrupts are not
//! so predictable. Production software should use a more sophisticated scheme
//! to arbitrate shared data access.

#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;

use gd32vf103_pac::{USART0, Peripherals};
use gd32vf103xx_hal::afio::AfioExt;
use gd32vf103xx_hal::gpio::GpioExt;
use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::serial::{Config, Serial, Tx};
use gd32vf103xx_hal::time::U32Ext;
use riscv::register::{mie, mstatus};


/// Static storage for "transmit" portion of `USART0`
static mut TX: Option<Tx<USART0>> = None;

/// Prints formatted text to `TX`
macro_rules! tx_print {
    ($($arg:tt)*) => {
        if let Some(tx) = unsafe { TX.as_mut() } {
            let _ = write!(tx, $($arg)*);
        }
    };
}

/// Prints a line of formatted text to `TX`
macro_rules! tx_println {
    ($fmt:expr) => (tx_print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (tx_print!(concat!($fmt, "\r\n"), $($arg)*));
}

/// Static storage for `CTIMER`
static mut CTIMER: Option<gd32vf103_pac::CTIMER> = None;

/// Global interrupt handler
///
/// See *riscv-rt* docs for more information about defining interrupt and
/// exception handlers.
#[export_name = "DefaultHandler"]
fn handle_interrupt() {

    tx_println!("hello from interrupt handler");

    if let Some(ctimer) = unsafe { CTIMER.as_ref() } {
        ctimer.msip.modify(| _, w| w.msip().clear_bit());
    }
}


#[riscv_rt::entry]
fn main() -> ! {

    let peripherals = Peripherals::take().unwrap();

    let mut rcu = peripherals.RCU.configure()
        .freeze();

    let mut afio = peripherals.AFIO.constrain(&mut rcu);

    let gpioa = peripherals.GPIOA.split(&mut rcu);
    let tx = gpioa.pa9;
    let rx = gpioa.pa10;

    let config = Config::default()
        .baudrate(115_200.bps());
    let serial = Serial::new(peripherals.USART0, (tx, rx), config, &mut afio, &mut rcu);
    let (tx, _) = serial.split();

    unsafe {
        TX.replace(tx);
        CTIMER = Some(peripherals.CTIMER);
    }

    tx_println!("enabling software interrupts");
    unsafe {
        mie::set_msoft();
        mstatus::set_mie();
    }

    tx_println!("generating software interrupt");
    unsafe {
        CTIMER.as_ref()
            .map(|c| c.msip.modify(|_, w| w.msip().set_bit()));
    }

    tx_println!("done");

    loop {}
}
