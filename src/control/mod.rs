use std::fs;

use esp32rs::{esp32::ESP32, servo::MG90S, symbols::Symbols};
use gorilla_physics::{
    hybrid::{articulated::Articulated, control::ArticulatedController},
    types::Float,
};
use nalgebra::{DVector, dvector};

pub mod pid;

pub struct SesameESP32Controller {
    pub esp32: ESP32,
    pub mg90s: MG90S,
}

impl SesameESP32Controller {
    pub async fn new() -> Self {
        let mut symbols = Symbols::new(); // symbols for printing

        let rom1_data: Vec<u8>;
        let rom0_data: Vec<u8>;
        let bootloader_data: Vec<u8>;
        let partition_table_data: Vec<u8>;
        let app_data: Vec<u8>;

        #[cfg(not(target_arch = "wasm32"))]
        {
            rom1_data = fs::read("rom/wokwi/rom1.bin").unwrap();
            rom0_data = fs::read("rom/wokwi/rom0.bin").unwrap();
            symbols.add("rom/symbols.txt");

            bootloader_data = fs::read("app/build/app.ino.bootloader.bin").unwrap();
            partition_table_data = fs::read("app/build/app.ino.partitions.bin").unwrap();
            app_data = fs::read("app/build/app.ino.bin").unwrap();
            symbols.add("app/build/symbols.txt");
            symbols.add("app/bootloader_symbols.txt");
        }

        #[cfg(target_arch = "wasm32")]
        {
            use gorilla_physics::interface::util::read_web_file_bytes;
            rom1_data = read_web_file_bytes("rom/wokwi/rom1.bin").await;
            rom0_data = read_web_file_bytes("rom/wokwi/rom0.bin").await;
            bootloader_data = read_web_file_bytes("app/build/app.ino.bootloader.bin").await;
            partition_table_data = read_web_file_bytes("app/build/app.ino.partitions.bin").await;
            app_data = read_web_file_bytes("app/build/app.ino.bin").await;
        }

        let esp32 = ESP32::new(
            rom1_data,
            rom0_data,
            bootloader_data,
            partition_table_data,
            app_data,
            symbols,
        );

        Self {
            esp32,
            mg90s: MG90S::new(),
        }
    }
}

impl ArticulatedController for SesameESP32Controller {
    fn step(&mut self, dt: Float, articulated: &Articulated) {
        let Hz = 240_000_000 as Float; // 240 Mhz
        let n_steps = (dt * Hz) as usize;

        let mut count = 0;
        let max_count = 100;
        for _ in 0..n_steps {
            self.esp32.step();

            count += 1;
            if count == max_count {
                count = 0;
                if let Some(pin2) = self.esp32.read_pin(2) {
                    self.mg90s.step(max_count as Float / Hz, pin2);
                }
            }
        }
    }

    fn control(&mut self, articulated: &Articulated, input: &Vec<Float>) -> DVector<Float> {
        self.mg90s.angle = articulated.q()[0];
        self.mg90s.vel = articulated.v()[0];
        let torque = self.mg90s.torque();
        dvector![torque]
    }

    fn debug_data(&self) -> Float {
        if self.mg90s.pin_prev { 1. } else { 0. }
    }
}
