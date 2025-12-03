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

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut led1 = pins.d9.into_output().into_pwm(&timer1);
    let mut led2 = pins.d10.into_output().into_pwm(&timer1);
    let mut led3 = pins.d11.into_output().into_pwm(&timer2);

    let mut delay = arduino_hal::Delay::new();

    led1.enable();
    led2.enable();
    led3.enable();

    loop {
        fade(&mut led1, &mut delay);
        fade(&mut led2, &mut delay);
        fade(&mut led3, &mut delay);
    }

}

fn fade(led: &mut impl SetDutyCycle, delay: &mut arduino_hal::Delay)-> () {
     for pct in 0..=50 {
        led.set_duty_cycle_percent(pct).ok();
        delay.delay_ms(60);
     }   
     for pct in (0..=50).rev() {
        led.set_duty_cycle_percent(pct).ok();
        delay.delay_ms(60);
     }
     led.set_duty_cycle_percent(0).ok();
    }
