#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs,
    digital::{OutputPin, StatefulOutputPin},
};
use microbit::{hal::Timer, Board};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let _ = board.display_pins.col3.set_low();
    let mut row3 = board.display_pins.row3;

    loop {
        row3.toggle().ok();
        timer.delay_ms(500);
    }
}