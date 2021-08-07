//! Reimplementation of DrPetter's "sfxr" sound effect generator.
//!
//! This crate provides tools for creating quick placeholder sound effects. The effects are
//! primarily intended to be used in quickly made games.
//!
//! Sound effects are first defined as `Sample` values, which has many fields for tuning the
//! properties of the resulting sound. A simple base Sample can be created with `Sample::new`,
//! but other constructors for various purposes are provided for quick random Samples.
//!
//! Next, a `Generator` is constructed to handle filling a sound buffer with data.
//!
//! # Examples
//!
//! Generating a smooth sine wave into a buffer
//!
//! ``` rust
//! use sfxr::Sample;
//! use sfxr::Generator;
//! use sfxr::WaveType;
//! let mut sample = Sample::new();
//! sample.wave_type = WaveType::Sine;
//! let mut  generator = Generator::new(sample);
//! let mut  buffer = [0.0; 44_100];
//! generator.generate(&mut buffer);
//! ```
//!
//! Generating a random explosion effect
//!
//! ``` rust
//! use sfxr::Sample;
//! use sfxr::Generator;
//! let sample = Sample::explosion(None);
//! let mut  generator = Generator::new(sample);
//! let mut  buffer = [0.0; 44_100];
//! generator.generate(&mut buffer);
//! ```

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

mod generator;

pub use generator::WaveType;

use generator::{Envelope, Filterable, HighLowPassFilter, Oscillator, Phaser};

/// Defines a sound effect configuration for a Generator
pub struct Sample {
    /// Oscillator wave type
    pub wave_type: WaveType,
    /// Oscillator base frequency. Value must be between `0.0` and `1.0`.
    pub base_freq: f64,
    /// Oscillator frequency limit. Value must be between `0.0` and `1.0`.
    pub freq_limit: f64,
    /// Oscillator frequency change over time. Value must be between `-1.0` and `1.0`.
    pub freq_ramp: f64,
    /// `freq_ramp` change over time. Value must be between `-1.0` and `1.0`.
    pub freq_dramp: f64,
    /// Oscillator square wave duty cycle. Value must be between `0.0` and `1.0`.
    pub duty: f32,
    /// Oscillator square wave duty cycle change over time. Value must be between `-1.0` and `1.0`.
    pub duty_ramp: f32,

    /// Vibrato strength. Value must be between `0.0` and `1.0`.
    pub vib_strength: f64,
    /// Vibrato speed. Value must be between `0.0` and `1.0`.
    pub vib_speed: f64,
    /// Vibrato delay. Value must be between `0.0` and `1.0`.
    pub vib_delay: f32,

    /// Duration of attack envelope. Value must be between `0.0` and `1.0`.
    pub env_attack: f32,
    /// Duration of sustain envelope. Value must be between `0.0` and `1.0`.
    pub env_sustain: f32,
    /// Duration of decay envelope. Value must be between `0.0` and `1.0`.
    pub env_decay: f32,
    /// Amount of "punch" in sustain envelope. Value must be between `-1.0` and `1.0`.
    pub env_punch: f32,

    /// Low pass filter resonance. Value must be between `0.0` and `1.0`.
    pub lpf_resonance: f32,
    /// Low pass filter cutoff frequency. Value must be between `0.0` and `1.0`.
    pub lpf_freq: f32,
    /// Low pass filter cutoff frequency change over time. Value must be between `-1.0` and `1.0`.
    pub lpf_ramp: f32,
    /// High pass filter cutoff frequency. Value must be between `0.0` and `1.0`.
    pub hpf_freq: f32,
    /// High pass filter cutoff frequency change over time. Value must be between `-1.0` and `1.0`.
    pub hpf_ramp: f32,

    /// Phaser temporal offset. Value must be between `-1.0` and `1.0`.
    pub pha_offset: f32,
    /// Phaser temporal offset change over time. Value must be between `-1.0` and `1.0`.
    pub pha_ramp: f32,

    /// Sample repeat speed. Value must be between `0.0` and `1.0`.
    pub repeat_speed: f32,

    /// Arpeggio interval. Value must be between `0.0` and `1.0`.
    pub arp_speed: f32,
    /// Arpeggio step in frequency. Value must be between `-1.0` and `1.0`.
    pub arp_mod: f64,
}

#[allow(clippy::new_without_default)]
impl Sample {
    /// Constructs a new Sample with default settings
    pub fn new() -> Sample {
        Sample {
            wave_type: WaveType::Square,
            base_freq: 0.3,
            freq_limit: 0.0,
            freq_ramp: 0.0,
            freq_dramp: 0.0,
            duty: 0.0,
            duty_ramp: 0.0,

            vib_strength: 0.0,
            vib_speed: 0.0,
            vib_delay: 0.0,

            env_attack: 0.4,
            env_sustain: 0.1,
            env_decay: 0.5,
            env_punch: 0.0,

            lpf_resonance: 0.0,
            lpf_freq: 1.0,
            lpf_ramp: 0.0,
            hpf_freq: 0.0,
            hpf_ramp: 0.0,

            pha_offset: 0.0,
            pha_ramp: 0.0,

            repeat_speed: 0.0,

            arp_speed: 0.0,
            arp_mod: 0.0,
        }
    }

    /// Asserts all fields' values to be within correct values
    fn assert_valid(&self) {
        assert!(
            self.base_freq >= 0.0 && self.base_freq <= 1.0,
            "base_freq must be between 0.0 and 1.0"
        );
        assert!(
            self.freq_limit >= 0.0 && self.freq_limit <= 1.0,
            "freq_limit must be between 0.0 and 1.0"
        );
        assert!(
            self.freq_ramp >= -1.0 && self.freq_ramp <= 1.0,
            "freq_ramp must be between -1.0 and 1.0"
        );
        assert!(
            self.freq_dramp >= 0.0 && self.freq_dramp <= 1.0,
            "freq_dramp must be between 0.0 and 1.0"
        );
        assert!(
            self.duty >= 0.0 && self.duty <= 1.0,
            "duty must be between 0.0 and 1.0"
        );
        assert!(
            self.duty_ramp >= -1.0 && self.duty_ramp <= 1.0,
            "duty_ramp must be between -1.0 and 1.0"
        );
        assert!(
            self.vib_strength >= 0.0 && self.vib_strength <= 1.0,
            "vib_strength must be between 0.0 and 1.0"
        );
        assert!(
            self.vib_speed >= 0.0 && self.vib_speed <= 1.0,
            "vib_speed must be between 0.0 and 1.0"
        );
        assert!(
            self.vib_delay >= 0.0 && self.vib_delay <= 1.0,
            "vib_delay must be between 0.0 and 1.0"
        );
        assert!(
            self.env_attack >= 0.0 && self.env_attack <= 1.0,
            "env_attack must be between 0.0 and 1.0"
        );
        assert!(
            self.env_sustain >= 0.0 && self.env_sustain <= 1.0,
            "env_sustain must be between 0.0 and 1.0"
        );
        assert!(
            self.env_decay >= 0.0 && self.env_decay <= 1.0,
            "env_decay must be between 0.0 and 1.0"
        );
        assert!(
            self.env_punch >= -1.0 && self.env_punch <= 1.0,
            "env_punch must be between -1.0 and 1.0"
        );
        assert!(
            self.lpf_resonance >= 0.0 && self.lpf_resonance <= 1.0,
            "lpf_resonance must be between 0.0 and 1.0"
        );
        assert!(
            self.lpf_freq >= 0.0 && self.lpf_freq <= 1.0,
            "lpf_freq must be between 0.0 and 1.0"
        );
        assert!(
            self.lpf_ramp >= -1.0 && self.lpf_ramp <= 1.0,
            "lpf_ramp must be between -1.0 and 1.0"
        );
        assert!(
            self.hpf_freq >= 0.0 && self.hpf_freq <= 1.0,
            "hpf_freq must be between 0.0 and 1.0"
        );
        assert!(
            self.hpf_ramp >= -1.0 && self.hpf_ramp <= 1.0,
            "hpf_ramp must be between -1.0 and 1.0"
        );
        assert!(
            self.pha_offset >= -1.0 && self.pha_offset <= 1.0,
            "pha_offset must be between -1.0 and 1.0"
        );
        assert!(
            self.pha_ramp >= -1.0 && self.pha_ramp <= 1.0,
            "pha_ramp must be between -1.0 and 1.0"
        );
        assert!(
            self.repeat_speed >= 0.0 && self.repeat_speed <= 1.0,
            "repeat_speed must be between 0.0 and 1.0"
        );
        assert!(
            self.arp_speed >= 0.0 && self.arp_speed <= 1.0,
            "arp_speed must be between 0.0 and 1.0"
        );
        assert!(
            self.arp_mod >= -1.0 && self.arp_mod <= 1.0,
            "arp_mod must be between -1.0 and 1.0"
        );
    }

    /// Changes Sample fields randomly by a little
    pub fn mutate(&mut self, seed: Option<u64>) {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));

        fn mutate_f64(rng: &mut SmallRng, v: &mut f64, min: f64, max: f64) {
            if rand_bool(rng, 1, 1) {
                *v = (*v + rand_f64(rng, -0.05, 0.05)).min(max).max(min);
            }
        }
        fn mutate_f32(rng: &mut SmallRng, v: &mut f32, min: f32, max: f32) {
            if rand_bool(rng, 1, 1) {
                *v = (*v + rand_f32(rng, -0.05, 0.05)).min(max).max(min);
            }
        }

        mutate_f64(rng, &mut self.base_freq, 0.0, 1.0);
        // Commented out in sfxr?
        // mutate_f64(rng, &mut self.freq_limit);
        mutate_f64(rng, &mut self.freq_ramp, -1.0, 1.0);
        mutate_f64(rng, &mut self.freq_dramp, 0.0, 1.0);
        mutate_f32(rng, &mut self.duty, 0.0, 1.0);
        mutate_f32(rng, &mut self.duty_ramp, -1.0, 1.0);
        mutate_f64(rng, &mut self.vib_strength, 0.0, 1.0);
        mutate_f64(rng, &mut self.vib_speed, 0.0, 1.0);
        mutate_f32(rng, &mut self.vib_delay, 0.0, 1.0);
        mutate_f32(rng, &mut self.env_attack, 0.0, 1.0);
        mutate_f32(rng, &mut self.env_sustain, 0.0, 1.0);
        mutate_f32(rng, &mut self.env_decay, 0.0, 1.0);
        mutate_f32(rng, &mut self.env_punch, -1.0, 1.0);
        mutate_f32(rng, &mut self.lpf_resonance, 0.0, 1.0);
        mutate_f32(rng, &mut self.lpf_freq, 0.0, 1.0);
        mutate_f32(rng, &mut self.lpf_ramp, -1.0, 1.0);
        mutate_f32(rng, &mut self.hpf_freq, 0.0, 1.0);
        mutate_f32(rng, &mut self.hpf_ramp, -1.0, 1.0);
        mutate_f32(rng, &mut self.pha_offset, -1.0, 1.0);
        mutate_f32(rng, &mut self.pha_ramp, 0.0, 1.0);
        mutate_f32(rng, &mut self.repeat_speed, 0.0, 1.0);
        mutate_f32(rng, &mut self.arp_speed, 0.0, 1.0);
        mutate_f64(rng, &mut self.arp_mod, -1.0, 1.0);
    }

    /// Constructs a new random "coin" or "item pickup" style sample using optional random seed
    pub fn pickup(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        s.base_freq = rand_f64(rng, 0.4, 0.9);
        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.0, 0.1);
        s.env_decay = rand_f32(rng, 0.1, 0.5);
        s.env_punch = rand_f32(rng, 0.3, 0.6);

        if rand_bool(rng, 1, 1) {
            s.arp_speed = rand_f32(rng, 0.5, 0.7);
            s.arp_mod = rand_f64(rng, 0.2, 0.6);
        }

        s
    }

    /// Constructs a new random "shoot" or "laser" style sample using optional random seed
    pub fn laser(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        let wave_types = {
            use WaveType::*;
            [Square, Square, Sine, Sine, Triangle]
        };
        s.wave_type = rand_element(rng, &wave_types);

        if rand_bool(rng, 1, 2) {
            s.base_freq = rand_f64(rng, 0.3, 0.9);
            s.freq_limit = rand_f64(rng, 0.0, 0.1);
            s.freq_ramp = rand_f64(rng, -0.35, -0.65);
        } else {
            s.base_freq = rand_f64(rng, 0.5, 1.0);
            s.freq_limit = (s.base_freq - rand_f64(rng, 0.2, 0.8)).max(0.2);
            s.freq_ramp = rand_f64(rng, -0.15, -0.35);
        }

        if rand_bool(rng, 1, 1) {
            s.duty = rand_f32(rng, 0.0, 0.5);
            s.duty_ramp = rand_f32(rng, 0.0, 0.2);
        } else {
            s.duty = rand_f32(rng, 0.4, 0.9);
            s.duty_ramp = rand_f32(rng, 0.0, -0.7);
        }

        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.1, 0.3);
        s.env_decay = rand_f32(rng, 0.0, 0.4);

        if rand_bool(rng, 1, 1) {
            s.env_punch = rand_f32(rng, 0.0, 0.3);
        }

        if rand_bool(rng, 1, 2) {
            s.pha_offset = rand_f32(rng, 0.0, 0.2);
            s.pha_ramp = -rand_f32(rng, 0.0, 0.2);
        }

        if rand_bool(rng, 1, 1) {
            s.hpf_freq = rand_f32(rng, 0.0, 0.3);
        }

        s
    }

    /// Constructs a new random "explosion" style sample using optional random seed
    pub fn explosion(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        s.wave_type = WaveType::Noise;

        if rand_bool(rng, 1, 1) {
            s.base_freq = rand_f64(rng, 0.1, 0.5);
            s.freq_ramp = rand_f64(rng, -0.1, 0.3);
        } else {
            s.base_freq = rand_f64(rng, 0.2, 0.9);
            s.freq_ramp = rand_f64(rng, -0.2, -0.4);
        }

        s.base_freq = s.base_freq.powi(2);

        if rand_bool(rng, 1, 4) {
            s.freq_ramp = 0.0;
        }

        if rand_bool(rng, 1, 2) {
            s.repeat_speed = rand_f32(rng, 0.3, 0.8);
        }

        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.1, 0.4);
        s.env_decay = rand_f32(rng, 0.0, 0.5);

        if rand_bool(rng, 1, 1) {
            s.pha_offset = rand_f32(rng, -0.3, 0.6);
            s.pha_ramp = rand_f32(rng, -0.3, 0.0);
        }

        s.env_punch = rand_f32(rng, 0.2, 0.8);

        if rand_bool(rng, 1, 1) {
            s.vib_strength = rand_f64(rng, 0.0, 0.7);
            s.vib_speed = rand_f64(rng, 0.0, 0.6);
        }

        if rand_bool(rng, 1, 2) {
            s.arp_speed = rand_f32(rng, 0.6, 0.9);
            s.arp_mod = rand_f64(rng, -0.8, 0.8);
        }

        s
    }

    /// Constructs a new random "powerup" style sample using optional random seed
    pub fn powerup(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        if rand_bool(rng, 1, 1) {
            s.wave_type = WaveType::Sine;
        } else {
            s.duty = rand_f32(rng, 0.0, 0.6);
        }

        s.base_freq = rand_f64(rng, 0.2, 0.5);

        if rand_bool(rng, 1, 1) {
            s.freq_ramp = rand_f64(rng, 0.1, 0.5);
            s.repeat_speed = rand_f32(rng, 0.4, 0.8);
        } else {
            s.freq_ramp = rand_f64(rng, 0.05, 0.25);

            if rand_bool(rng, 1, 1) {
                s.vib_strength = rand_f64(rng, 0.0, 0.7);
                s.vib_speed = rand_f64(rng, 0.0, 0.6);
            }
        }

        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.0, 0.4);
        s.env_decay = rand_f32(rng, 0.1, 0.5);

        s
    }

    /// Constructs a new random "hit" or "damage" style sample using optional random seed
    pub fn hit(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        s.wave_type = rand_element(rng, &[WaveType::Square, WaveType::Sine, WaveType::Noise]);

        if s.wave_type == WaveType::Square {
            s.duty = rand_f32(rng, 0.0, 0.6);
        }

        s.base_freq = rand_f64(rng, 0.2, 0.8);
        s.freq_ramp = rand_f64(rng, -0.3, -0.7);
        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.0, 0.1);
        s.env_decay = rand_f32(rng, 0.1, 0.3);

        if rand_bool(rng, 1, 1) {
            s.hpf_freq = rand_f32(rng, 0.0, 0.3);
        }

        s
    }

    /// Constructs a new random "jump" style sample using optional random seed
    pub fn jump(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        s.wave_type = WaveType::Square;
        s.duty = rand_f32(rng, 0.0, 0.6);
        s.base_freq = rand_f64(rng, 0.3, 0.6);
        s.freq_ramp = rand_f64(rng, 0.1, 0.3);
        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.1, 0.4);
        s.env_decay = rand_f32(rng, 0.1, 0.3);

        if rand_bool(rng, 1, 1) {
            s.hpf_freq = rand_f32(rng, 0.0, 0.3);
        }

        if rand_bool(rng, 1, 1) {
            s.lpf_freq = rand_f32(rng, 0.4, 1.0);
        }

        s
    }

    /// Constructs a new random "blip" or "menu navigation" style sample using optional random seed
    pub fn blip(seed: Option<u64>) -> Sample {
        let rng = &mut SmallRng::seed_from_u64(seed.unwrap_or(0));
        let mut s = Sample::new();

        s.wave_type = rand_element(rng, &[WaveType::Square, WaveType::Sine]);

        if s.wave_type == WaveType::Square {
            s.duty = rand_f32(rng, 0.0, 0.6);
        }

        s.base_freq = rand_f64(rng, 0.2, 0.6);
        s.env_attack = 0.0;
        s.env_sustain = rand_f32(rng, 0.1, 0.2);
        s.env_decay = rand_f32(rng, 0.0, 0.2);
        s.hpf_freq = 0.1;

        s
    }
}

/// Sound effect generator
///
/// Generates sound effect data according to a Sample into a buffer. The data can be generated in
/// multiple chunks, as the generator maintains its state from one call to `generate` to the next.
pub struct Generator {
    /// Generator settings
    pub sample: Sample,

    /// Sound effect volume. Default is `0.2`.
    pub volume: f32,
    oscillator: Oscillator,
    hlpf: HighLowPassFilter,
    envelope: Envelope,
    phaser: Phaser,
    rep_time: i32,
    rep_limit: i32,
}
impl Generator {
    /// Constructs a new Generator based on the provided Sample
    pub fn new(s: Sample) -> Generator {
        s.assert_valid();
        let wave_type = s.wave_type;
        let mut g = Generator {
            sample: s,
            volume: 0.2,
            oscillator: Oscillator::new(wave_type),
            hlpf: HighLowPassFilter::new(),
            envelope: Envelope::new(),
            phaser: Phaser::new(),
            rep_time: 0,
            rep_limit: 0,
        };

        g.reset();

        g
    }
    /// Fills `buffer` with sound effect data. Subsequent calls continue where the last left off.
    /// Call `reset` first to start generating from the beginning.
    pub fn generate(&mut self, buffer: &mut [f32]) {
        buffer.iter_mut().for_each(|buffer_value| {
            self.rep_time += 1;

            if self.rep_limit != 0 && self.rep_time >= self.rep_limit {
                self.rep_time = 0;
                self.restart();
            }

            self.oscillator.advance();
            self.envelope.advance();
            self.phaser.advance();

            let sample = self
                .oscillator
                .by_ref()
                .chain_filter(&mut self.envelope)
                .chain_filter(&mut self.hlpf)
                .chain_filter(&mut self.phaser)
                .take(8)
                .sum::<f32>()
                / 8.0;

            *buffer_value = (sample * self.volume).min(1.0).max(-1.0);
        });
    }
    /// Resets the generator to the beginning of the sound effect.
    pub fn reset(&mut self) {
        self.restart();
        self.envelope.reset(
            self.sample.env_attack,
            self.sample.env_sustain,
            self.sample.env_decay,
            self.sample.env_punch,
        );
        self.phaser
            .reset(self.sample.pha_offset, self.sample.pha_ramp);

        self.oscillator.reset_phase();
        self.oscillator
            .reset_vibrato(self.sample.vib_speed, self.sample.vib_strength);
        self.oscillator.reset_noise();

        self.rep_time = 0;
        self.rep_limit = ((1.0 - self.sample.repeat_speed).powi(2) * 20_000.0 * 32.0) as i32;

        if self.sample.repeat_speed == 0.0 {
            self.rep_limit = 0;
        }
    }
    /// Resets only the oscillator and band pass filter.
    fn restart(&mut self) {
        self.hlpf.reset(
            self.sample.lpf_resonance,
            self.sample.lpf_freq,
            self.sample.lpf_ramp,
            self.sample.hpf_freq,
            self.sample.hpf_ramp,
        );
        self.oscillator.reset(
            self.sample.wave_type,
            self.sample.base_freq,
            self.sample.freq_limit,
            self.sample.freq_ramp,
            self.sample.freq_dramp,
            self.sample.duty,
            self.sample.duty_ramp,
            self.sample.arp_speed,
            self.sample.arp_mod,
        );
    }
}

/// Generate a random `f32` using `rng` in the range [`from`...`until`).
fn rand_f32(rng: &mut SmallRng, from: f32, until: f32) -> f32 {
    from + (until - from) * rng.gen::<f32>()
}
/// Generate a random `f64` using `rng` in the range [`from`...`until`).
fn rand_f64(rng: &mut SmallRng, from: f64, until: f64) -> f64 {
    from + (until - from) * rng.gen::<f64>()
}
/// Generate a random `bool` using `rng` with `chance_true`:`chance_false` odds of being true.
fn rand_bool(rng: &mut SmallRng, chance_true: u32, chance_false: u32) -> bool {
    rng.gen::<u32>() % (chance_true + chance_false) < chance_true
}
/// Pick a random element from `slice` using `rng`.
fn rand_element<T: Copy>(rng: &mut SmallRng, slice: &[T]) -> T {
    slice[rng.gen::<u32>() as usize % slice.len()]
}
