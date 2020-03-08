//! Demonstrates flashing a single LED continuously

#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_pac::Peripherals;
use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::timer::Timer;
use nb::block;


#[riscv_rt::entry]
fn main() -> !
{
    let peripherals = Peripherals::take().unwrap();

    let mut rcu = peripherals.RCU.configure()
        .freeze();

    let gpioc = peripherals.GPIOC.split(&mut rcu);
    let mut led_1 = gpioc.pc13.into_push_pull_output();

    let mut timer = Timer::timer1(peripherals.TIMER1, 1.hz(), &mut rcu);

    loop
    {
        let _ = led_1.set_high();
        timer.start(1.hz());
        let _ = block!(timer.wait());

        let _ = led_1.set_low();
        timer.start(1.hz());
        let _ = block!(timer.wait());
    }
}
