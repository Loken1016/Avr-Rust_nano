#![no_std]
#![no_main]

use panic_halt as _;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
let dp = arduino_hal::Peripherals::take().unwrap();
let pins = arduino_hal::pins!(dp);

let mut serial  = arduino_hal::default_serial!(dp, pins, 115200);

uwriteln!(&mut serial, "Hola mundo!\r").unwrap();
loop {}

}