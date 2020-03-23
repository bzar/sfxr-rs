extern crate cpal;
extern crate rand;
extern crate sfxr;

use cpal::traits::{EventLoopTrait, HostTrait};
use rand::rngs::SmallRng;
use rand::{FromEntropy, RngCore};
use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
};

/// Manages the audio.
pub struct Audio {
    generator: Arc<Mutex<Option<sfxr::Generator>>>,
}

impl Audio {
    /// Instantiate a new audio object without a generator.
    pub fn new() -> Self {
        Self {
            generator: Arc::new(Mutex::new(None)),
        }
    }

    /// Play a sample.
    pub fn play(&mut self, sample: sfxr::Sample) {
        let mut generator = self.generator.lock().unwrap();
        *generator = Some(sfxr::Generator::new(sample));
    }

    /// Start a thread which will emit the audio.
    pub fn run(&mut self) {
        let generator = self.generator.clone();

        thread::spawn(|| {
            // Setup the audio system
            let host = cpal::default_host();
            let event_loop = host.event_loop();

            let device = host
                .default_output_device()
                .expect("no output device available");

            // This is the only format sfxr supports
            let format = cpal::Format {
                channels: 1,
                sample_rate: cpal::SampleRate(44_100),
                data_type: cpal::SampleFormat::F32,
            };

            let stream_id = event_loop
                .build_output_stream(&device, &format)
                .expect("could not build output stream");

            event_loop
                .play_stream(stream_id)
                .expect("could not play stream");

            event_loop.run(move |stream_id, stream_result| {
                let stream_data = match stream_result {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                        return;
                    }
                };

                match stream_data {
                    cpal::StreamData::Output {
                        buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                    } => match *generator.lock().unwrap() {
                        Some(ref mut generator) => generator.generate(&mut buffer),
                        None => {
                            for elem in buffer.iter_mut() {
                                *elem = 0.0;
                            }
                        }
                    },
                    _ => panic!("output type buffer can not be used"),
                }
            });
        });
    }
}

fn main() {
    let mut sample = sfxr::Sample::new();
    sample.mutate(Some(SmallRng::from_entropy().next_u64()));

    let mut audio = Audio::new();

    // Spawn a background thread where an audio device is opened with cpal
    audio.run();

    // Play the sample
    audio.play(sample);

    std::thread::sleep(Duration::from_millis(1_000));
}
