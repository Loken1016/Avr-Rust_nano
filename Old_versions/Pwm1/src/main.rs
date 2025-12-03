#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;

fn fade(led: &mut impl SetDutyCycle, delay:&mut impl DelayNs) -> ! { 
    loop {
        for pct in (0..=100).rev() {        
        led.set_duty_cycle_percent(pct).unwrap();
        delay.delay_ms(20);
        }
        for pct in 0..=100 {
        led.set_duty_cycle_percent(pct).unwrap();
        delay.delay_ms(20);
        }
    }

}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();

    let mut delay = arduino_hal::Delay::new();
    fade(&mut pwm_led, &mut delay);
}



