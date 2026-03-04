use std::fs;

use esp32rs::{
    esp32::{CPU_SLOWDOWN_FACTOR, ESP32},
    servo::MG90S,
    symbols::Symbols,
};
use gorilla_physics::{
    hybrid::{articulated::Articulated, control::ArticulatedController},
    types::Float,
};
use nalgebra::{DVector, dvector};

pub mod pid;

pub struct SesameESP32Controller {
    pub esp32: ESP32,
    // pub mg90s: MG90S,
    pub mg90s: [MG90S; 8],
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

            bootloader_data = fs::read("sesame/build/sesame.ino.bootloader.bin").unwrap();
            partition_table_data = fs::read("sesame/build/sesame.ino.partitions.bin").unwrap();
            app_data = fs::read("sesame/build/sesame.ino.bin").unwrap();
            symbols.add("sesame/build/symbols.txt");
            symbols.add("sesame/bootloader_symbols.txt");
        }

        #[cfg(target_arch = "wasm32")]
        {
            use gorilla_physics::interface::util::read_web_file_bytes;
            rom1_data = read_web_file_bytes("rom/wokwi/rom1.bin").await;
            rom0_data = read_web_file_bytes("rom/wokwi/rom0.bin").await;
            bootloader_data = read_web_file_bytes("sesame/build/sesame.ino.bootloader.bin").await;
            partition_table_data =
                read_web_file_bytes("sesame/build/sesame.ino.partitions.bin").await;
            app_data = read_web_file_bytes("sesame/build/sesame.ino.bin").await;
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
            mg90s: [MG90S::new(); 8],
        }
    }
}

impl ArticulatedController for SesameESP32Controller {
    fn step(&mut self, dt: Float, articulated: &Articulated) {
        let Hz = (240_000_000 / CPU_SLOWDOWN_FACTOR) as Float; // 240 Mhz
        let n_steps = (dt * Hz) as usize;

        let pins = [15, 2, 23, 19, 4, 16, 17, 18];

        let mut count = 0;
        let max_count = 1; // 100
        for _ in 0..n_steps {
            self.esp32.step();

            count += 1;
            if count == max_count {
                count = 0;
                for i in 0..self.mg90s.len() {
                    if let Some(pin) = self.esp32.read_pin(pins[i]) {
                        self.mg90s[i].step(max_count as Float / Hz, pin);
                    }
                }
            }
        }
    }

    fn control(&mut self, articulated: &Articulated, input: &Vec<Float>) -> DVector<Float> {
        let mut torques = vec![];
        for i in 0..self.mg90s.len() {
            self.mg90s[i].angle = articulated.q()[i];
            self.mg90s[i].vel = articulated.v()[i];
            let torque = self.mg90s[i].torque();
            torques.push(torque);
        }

        // if let Some(command_angle) = self.mg90s.command_angle {
        //     println!("pulse: {}", self.mg90s.pulse_width);
        //     println!("target angle: {}", command_angle);
        // }
        DVector::from_vec(torques)
    }

    fn debug_data(&self) -> Float {
        if self.mg90s[0].pin_prev { 1. } else { 0. }
    }
}
