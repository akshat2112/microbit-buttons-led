#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let _col = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);
    let mut row = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);

    loop {
        row.set_high();
        Timer::after_millis(300).await;
        row.set_low();
        Timer::after_millis(300).await;
    }
}