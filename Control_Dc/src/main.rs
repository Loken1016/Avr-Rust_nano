#![no_std]
#![no_main]

use avr_device::atmega328p;
use panic_halt as _;
use core::ptr;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Obtener perifericos
    let dp = atmega328p::Peripherals::take().unwrap();

    // --- Configurar pines D8 y D7 como salida ---
    unsafe {
        // DDRB bits 0=D8, 1=D9 → DDRB0=PB0=D8, DDRB1=PB1=D9
        dp.PORTB.ddr.write(|w| w.bits(0b00000011)); 
    }

    // --- Configurar PWM en D9 (OC1A, Timer1, Fast PWM 8-bit) ---
    unsafe {
        // WGM10=1 (modo Fast PWM 8-bit), COM1A1=1 (set OC1A on compare match)
        dp.TC1.tccr1a.write(|w| w.bits(0b10000001));
        dp.TC1.tccr1b.write(|w| w.bits(0b00000011)); // CS11=1, CS10=1 → prescaler 64
        dp.TC1.ocr1a.write(|w| w.bits(128)); // 50% duty
    }

    // --- Variables para temporización ---
    let mut forward = true;
    let mut speed_high = false;
    let mut last_change = 0u32;

    loop {
        // Leer millis() desde Timer0 (simula arduino_hal::millis)
        let ms = unsafe { ptr::read_volatile(0x084 as *const u32) }; // Ejemplo: necesitas implementar un contador de millis

        if ms.wrapping_sub(last_change) >= 2000 {
            last_change = ms;

            // Cambiar dirección
            unsafe {
                if forward {
                    ptr::write_volatile(0x25 as *mut u8, 0b00000001); // PORTB0=HIGH, D8
                } else {
                    ptr::write_volatile(0x25 as *mut u8, 0b00000010); // PORTB1=HIGH, D9
                }

                // Cambiar velocidad
                let duty = if speed_high { 255 } else { 128 };
                dp.TC1.ocr1a.write(|w| w.bits(duty));
            }

            speed_high = !speed_high;
            if !speed_high {
                forward = !forward;
            }
        }
    }
}
