#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::*;
use embedded_hal::pwm::SetDutyCycle;
use embedded_hal::delay::DelayNs;

// Pwm con un led

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    
    let mut led = pins.d5.into_output().into_pwm(&timer0);
    let mut delay = arduino_hal::Delay::new();

    led.enable();

    loop {
        fade(&mut led,&mut delay);
    }
}

fn fade(led: &mut impl SetDutyCycle, delay: &mut arduino_hal::Delay) -> () {
     for pct in (0..=100).rev() {
         led.set_duty_cycle_percent(pct).unwrap();
         delay.delay_ms(20);
     }   
     for pct in 0..=100 {
         led.set_duty_cycle_percent(pct).unwrap();
         delay.delay_ms(20);
     }
    }
