use cpal::{
    traits::{
        DeviceTrait,
        HostTrait,
        StreamTrait,
    },
    StreamConfig,
};

use std::io::{self, Read};

use crate::input::stt;

const STT_SIZE: usize = 64000;

pub fn run_audio(){
    let stt = stt::SpeechToText::new_instance();

    let host = cpal::default_host();
    let device = host.default_input_device().expect("Unable to get default input device");
    let config = StreamConfig{
        channels: 1,
        sample_rate: cpal::SampleRate{0: 16000},
        buffer_size: cpal::BufferSize::Fixed(160),
    };
    
    let mut data_arr: Vec<f32> = Vec::with_capacity(STT_SIZE);
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo|{
            data_arr.extend(data);
            if data_arr.len() >= STT_SIZE{
                let stt_result = stt.interpret_text(data_arr.clone());
                if !stt_result.contains("[BLANK_AUDIO]"){
                    println!("{stt_result}");
                }
                data_arr = Vec::with_capacity(STT_SIZE);
            }
        },
        move |err|{
            panic!("{:?}", err);
        },
        None,
    ).expect("Could not create stream");
    stream.play().expect("Could not play stream");
    pause();
}

fn pause(){
    io::stdin().read(&mut [0]).unwrap();
}
