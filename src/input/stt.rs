use whisper_rs::{
    SamplingStrategy,
    FullParams,
    WhisperContext,
    WhisperContextParameters,
};

use std::{
    sync::mpsc::{
        self,
        Receiver,
        SyncSender,
    },
    thread::{
        self,
        JoinHandle,
    },
    time::Duration,
};

pub struct SpeechToText{
    _stt_thread: JoinHandle<()>,
    audio_sender: SyncSender<Vec<f32>>,
    text_receiver: Receiver<String>,
    loop_end_signal: SyncSender<u8>,
}

impl SpeechToText{
    pub fn new_instance() -> Self{
        let (audio_sender, audio_receiver) = mpsc::sync_channel::<Vec<f32>>(0);
        let (text_sender, text_receiver) = mpsc::sync_channel::<String>(0);
        let (loop_end_signal, loop_end_receiver) = mpsc::sync_channel::<u8>(0);
        let _stt_thread = thread::spawn(move ||{
            let loop_sleep_duration = Duration::from_millis(10);

            let context = WhisperContext::new_with_params(
                "./ggml-model-whisper-base.en.bin",
                WhisperContextParameters::default()
            ).expect("Failed to load model");

            let mut state = context.create_state().expect("Failed to create state");


            'mainloop: loop{
                let loop_end_try_recv = loop_end_receiver.try_recv();
                let _loop_end = match loop_end_try_recv{
                    Ok(_end) => {
                        println!("Ending speech-to-text engine");
                        break 'mainloop
                    },
                    Err(_e) => (),
                };

                let audio_try_recv = audio_receiver.try_recv();
                let audio_sample = match audio_try_recv {
                    Ok(sample) => sample,
                    Err(_e) => {
                        thread::sleep(loop_sleep_duration);
                        continue 'mainloop
                    },
                };

                let mut params = FullParams::new(SamplingStrategy::Greedy {best_of: 1});

                params.set_n_threads(8);
                params.set_language(Some("en"));
                params.set_print_special(false);
                params.set_print_progress(false);
                params.set_print_realtime(false);
                params.set_print_timestamps(false);

                let num_segments = state.full_n_segments().expect("Failed to get segments");

                state.full(params, &audio_sample).expect("Failed to run model");

                let mut segment_text: String = String::from("");

                for i in 0..num_segments{
                    let segment = state.full_get_segment_text(i).expect("Failed to get segment");
                    segment_text = format!("{}{}", segment_text, segment);
                }
                text_sender.send(segment_text).unwrap();
            }
        });

        return Self{
            _stt_thread,
            audio_sender,
            text_receiver,
            loop_end_signal,
        };
    }

    pub fn interpret_text(&self, audio_sample: Vec<f32>) -> String{
        self.audio_sender.send(audio_sample).unwrap();
        loop{
            let text_try_recv = self.text_receiver.try_recv();
            return match text_try_recv{
                Ok(return_text) => return_text,
                Err(_e) => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                },
            }
        }
    }
    pub fn end_stt(&self){
        self.loop_end_signal.send(0).unwrap();
    }
}
