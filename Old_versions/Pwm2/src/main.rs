#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut led1 = pins.d5.into_output().into_pwm(&timer0);
    led1.enable();

    let mut led2 = pins.d6.into_output().into_pwm(&timer0);
    led2.enable();

    let mut led3 = pins.d9.into_output().into_pwm(&timer1);
    led3.enable();

    let mut delay = arduino_hal::Delay::new();

    loop {
        led3.set_duty_cycle_percent(0).ok();

        for pct in 0..=100 {
            led1.set_duty_cycle_percent(pct).ok();
            delay.delay_ms(15);
        }

        for pct in (0..=100).rev() {
            led1.set_duty_cycle_percent(pct).ok();
            delay.delay_ms(15);
        }
        led1.set_duty_cycle_percent(0).ok();

        for pct in 0..=100 {
            led2.set_duty_cycle_percent(pct).ok();
            delay.delay_ms(15);
        }
        for pct in (0..=100).rev() {
            led2.set_duty_cycle_percent(pct).ok();
            delay.delay_ms(15);
        }
        led2.set_duty_cycle_percent(0).ok();

        for pct in 0..=100 {
            led3.set_duty_cycle_percent(pct).ok();
            delay.delay_ms(15);
        }
        for pct in (0..=100).rev() {
            led3.set_duty_cycle_percent(pct).ok();
            delay.delay_ms(15);
        }
    }
}
