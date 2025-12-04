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
    let mut led1 = pins.d7.into_output();
    let mut led2 = pins.d8.into_output();
    let boton = pins.a0.into_pull_up_input();
    
    led3.enable();
    


    loop { 
        if boton.is_low() {
            led3.set_duty(led3.get_max_duty());
            led1.set_high();
            led2.set_low();
        }
        else {
            led3.set_duty(0);
            led1.set_low();
            led2.set_low();
        }
    }
}