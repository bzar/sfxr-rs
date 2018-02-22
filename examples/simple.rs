extern crate sfxr;
extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct Sample<'a> {
    volume: f32,
    data: &'a [f32],
    pos: usize
}

impl<'a> AudioCallback for Sample<'a> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.data.get(self.pos).unwrap_or(&0.0) * self.volume;
            self.pos += 1;
        }
    }
}
fn main() {
    let mut buffer: [f32; 44_100] = [0.0; 44_100];
    let sample = sfxr::Sample::new();
    let mut generator = sfxr::Generator::new(sample);
    generator.generate(&mut buffer);

    //buffer.iter().for_each(|v| println!("{}", v));
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),
        samples: None
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("# {:?}", spec);

        Sample {
            volume: 0.25,
            data: &buffer,
            pos: 0
        }
    }).unwrap();

    device.resume();
    std::thread::sleep(Duration::from_millis(1_000));
}
