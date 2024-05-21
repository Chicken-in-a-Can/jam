use std::{
    fs::{
        self,
        File,
    },
    io::Read,
};

use cpal::{
    traits::{
        DeviceTrait,
        HostTrait,
        StreamTrait,
    },
    StreamConfig,
};

const PI: f32 = 3.14159265358979323846;

struct Instrument{
    name: String,
    num_overtones: u8,
    weights: Vec<u8>,
    distortion_quant: u8,
}

impl Instrument{
    fn load_instrument(instrument_name: String) -> Self{
        let file_location = format!("./{}.int", instrument_name);
        let mut file = File::open(&file_location).expect("Could not open file");
        let metadata = fs::metadata(&file_location).expect("Could not get metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("Buffer overflow");

        let distortion_quant = buffer[0];
        let num_overtones = buffer[1];
        let mut weights: Vec<u8> = vec![0; num_overtones as usize];

        for weight in 2..(num_overtones as usize + 2){
            weights[weight - 2] = buffer[weight];
        }

        return Self{
            name: instrument_name,
            num_overtones,
            weights,
            distortion_quant,
        };
    }

    pub fn play(root_frequency: f32, duration_s: f32, sample_rate: usize) -> Vec<f32>{
        let samples: usize = (sample_rate as f32 * duration_s) as usize;

        for sample in 0..samples{
            
        }
    }

    fn play_at_time(&self, root_frequency: f32, location: f32) -> f32{
        let mut simplified_location = location % (2.0 * PI);
        let sign: f32 = if simplified_location > PI{
            -1.0
        } else{
            1.0
        };
        simplified_location %= PI;

        let mut numerators: Vec<f32> = Vec::with_capacity(self.num_overtones as usize);
        let mut denominators: Vec<f32> = Vec::with_capacity(self.num_overtones as usize);

        for overtone in 0..(self.num_overtones as usize){
            let frequency_location: f32 = simplified_location * (root_frequency * (1 + overtone) as f32);
            numerators[overtone] = sign * (self.weights[overtone] as f32) * 16.0 * frequency_location * (PI - frequency_location);
            denominators[overtone] = (5.0 * PI * PI) - (4.0 * frequency_location * (PI - frequency_location));
        }

        let mut output: f32 = 0.0;
        for overtone in 0..(self.num_overtones as usize){
            output += numerators[overtone] / denominators[overtone];
        };

        return output;
    }
}
