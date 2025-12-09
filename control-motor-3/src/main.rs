#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut con = pins.d9.into_output().into_pwm(&timer1);
    let _dir1 = pins.d7.into_output().set_high();
    let _dir2 = pins.d8.into_output().set_low();
    let subir = pins.a0.into_pull_up_input();
    let bajar = pins.a1.into_pull_up_input();

    let mut vel:u8 = 0; 
    con.enable();

    loop {
        if subir.is_low() && vel < 255 {
            vel += 5;
        }
        if bajar.is_low() && vel > 0 {
            vel -= 5;
        }
        con.set_duty(vel);

        arduino_hal::delay_ms(100);
    }
}

