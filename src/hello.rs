//! Demonstrates sending "Hello, world!" over a serial connection

#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;

use gd32vf103_pac::Peripherals;
use gd32vf103xx_hal::gpio::GpioExt;
use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::serial::{Config, Serial};
use gd32vf103xx_hal::time::U32Ext;


#[riscv_rt::entry]
fn main() -> !
{
    let peripherals = Peripherals::take().unwrap();

    let mut rcu = peripherals.RCU.configure()
        .freeze();

    let gpioa = peripherals.GPIOA.split(&mut rcu);
    let tx = gpioa.pa9.into_alternate_push_pull();
    let rx = gpioa.pa10.into_floating_input();

    let config = Config::default()
        .baudrate(115_200.bps());
    let serial = Serial::usart0(peripherals.USART0, (tx, rx), config, &mut rcu);
    let (mut tx, _) = serial.split();

    let _ = write!(tx, "Hello, world!\r\n");

    loop {}
}
