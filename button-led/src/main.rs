#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use {defmt_rtt as _, panic_probe as _};

enum LedState {
    On,
    Off,
}

static CHANNEL: Channel<ThreadModeRawMutex, LedState, 1> = Channel::new();

#[embassy_executor::task(pool_size = 2)]
async fn button_task(n: usize, mut button_pin: Input<'static>) {
    loop {
        button_pin.wait_for_any_edge().await;

        if button_pin.is_low() {
            info!("Button {:?} pressed!", n);
            CHANNEL.send(LedState::On).await;
            info!("LED on!");
        } else {
            info!("Button {:?} released!", n);
            CHANNEL.send(LedState::Off).await;
            info!("LED off!");
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let _col = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);
    let mut led = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);

    info!("Starting!");
    let btn1 = Input::new(p.P0_14, Pull::Up);
    let btn2 = Input::new(p.P0_23, Pull::Up);

    unwrap!(spawner.spawn(button_task(1, btn1)));
    unwrap!(spawner.spawn(button_task(2, btn2)));

    loop {
        match CHANNEL.receive().await {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}
