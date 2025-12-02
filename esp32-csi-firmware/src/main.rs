#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    prelude::*,
};
use esp_println::println;

#[esp_hal::entry]
fn main() -> ! {
    let delay = Delay::new();

    println!("\n\n=== ESP32-C3 CSI Firmware ===");
    println!("Firmware initialized");
    println!("Starting CSI data transmission...\n");

    let mut measurement_count = 0u32;

    loop {
        // Send CSI-like data over serial
        // Format: +CSI:channel,bandwidth,rssi,noise_floor,subcarriers
        println!(
            "+CSI:6,20,-{},95,52",
            40 + (measurement_count % 20)
        );
        
        measurement_count += 1;
        delay.delay_millis(100);
    }
}
