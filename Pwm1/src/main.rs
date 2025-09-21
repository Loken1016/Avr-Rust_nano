#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;

// se define la funcion fade que se encargara de llevar el bucle encargado de subir y bajar el brillo
fn fade(led: &mut impl SetDutyCycle, delay:&mut impl DelayNs) -> ! {
//dentro de los parentesis se establece que led usara setDutyCycle y delay usara DelayNs, -> ! indica que fade no devolvera nada y se convertira en un bucle infinito 
    loop {
        for pct in (0..=100).rev() {
// ,rev() indica que el bucle contara de 100 hasta cero            
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
// dp se encargara de detectar los perifericos como pines y contadores del arduino y pins se encargara de llamar pines directamente 
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    

//  timer0 se encarga de crear un nuevo timer llamando al timer0 del arduino en una escala 64, el timer 0 se encarga de controlar los pines 5 y 6
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);


 // Configura el pin 5 como salida PWM usando el timer0
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();

//  delay se encarga de crear un nuevo delay ligandolo al led y al bucle de encendido y apagado
    let mut delay = arduino_hal::Delay::new();
    fade(&mut pwm_led, &mut delay);
}
