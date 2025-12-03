#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut led3 = pins.d9.into_output().into_pwm(&timer1);
    led3.enable();
    led3.set_duty(led3.get_max_duty() );

    let _led1 = pins.d7.into_output().set_high();
    let _led2 = pins.d8.into_output().set_low() ;


    loop { }
}
