extern crate rand;

use rand::{Rng};

mod generator;

pub use generator::WaveType;

use generator::{Envelope, HighLowPassFilter, Phaser, Oscillator, Filterable};

pub struct Sample {
    wave_type: WaveType,
    pub base_freq: f64,
    pub freq_limit: f64,
    pub freq_ramp: f64,
    pub freq_dramp: f64,
    pub duty: f32,
    pub duty_ramp: f32,

    pub vib_strength: f64,
    pub vib_speed: f64,
    pub vib_delay: f32,

    pub env_attack: f32,
    pub env_sustain: f32,
    pub env_decay: f32,
    pub env_punch: f32,

    pub lpf_resonance: f32,
    pub lpf_freq: f32,
    pub lpf_ramp: f32,
    pub hpf_freq: f32,
    pub hpf_ramp: f32,

    pub pha_offset: f32,
    pub pha_ramp: f32,

    pub repeat_speed: f32,

    pub arp_speed: f32,
    pub arp_mod: f64
}

impl Sample {
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
            arp_mod: 0.0
        }
    }
    fn assert_valid(&self) {
        assert!(self.base_freq >= 0.0 && self.base_freq <= 1.0, "base_freq must be between 0.0 and 1.0");
        assert!(self.freq_limit >= 0.0 && self.freq_limit <= 1.0, "freq_limit must be between 0.0 and 1.0");
        assert!(self.freq_ramp >= -1.0 && self.freq_ramp <= 1.0, "freq_ramp must be between -1.0 and 1.0");
        assert!(self.freq_dramp >= 0.0 && self.freq_dramp <= 1.0, "freq_dramp must be between 0.0 and 1.0");
        assert!(self.duty >= 0.0 && self.duty <= 1.0, "duty must be between 0.0 and 1.0");
        assert!(self.duty_ramp >= -1.0 && self.duty_ramp <= 1.0, "duty_ramp must be between -1.0 and 1.0");
        assert!(self.vib_strength >= 0.0 && self.vib_strength <= 1.0, "vib_strength must be between 0.0 and 1.0");
        assert!(self.vib_speed >= 0.0 && self.vib_speed <= 1.0, "vib_speed must be between 0.0 and 1.0");
        assert!(self.vib_delay >= 0.0 && self.vib_delay <= 1.0, "vib_delay must be between 0.0 and 1.0");
        assert!(self.env_attack >= 0.0 && self.env_attack <= 1.0, "env_attack must be between 0.0 and 1.0");
        assert!(self.env_sustain >= 0.0 && self.env_sustain <= 1.0, "env_sustain must be between 0.0 and 1.0");
        assert!(self.env_decay >= 0.0 && self.env_decay <= 1.0, "env_decay must be between 0.0 and 1.0");
        assert!(self.env_punch >= -1.0 && self.env_punch <= 1.0, "env_punch must be between -1.0 and 1.0");
        assert!(self.lpf_resonance >= 0.0 && self.lpf_resonance <= 1.0, "lpf_resonance must be between 0.0 and 1.0");
        assert!(self.lpf_freq >= 0.0 && self.lpf_freq <= 1.0, "lpf_freq must be between 0.0 and 1.0");
        assert!(self.lpf_ramp >= -1.0 && self.lpf_ramp <= 1.0, "lpf_ramp must be between -1.0 and 1.0");
        assert!(self.hpf_freq >= 0.0 && self.hpf_freq <= 1.0, "hpf_freq must be between 0.0 and 1.0");
        assert!(self.hpf_ramp >= -1.0 && self.hpf_ramp <= 1.0, "hpf_ramp must be between -1.0 and 1.0");
        assert!(self.pha_offset >= -1.0 && self.pha_offset <= 1.0, "pha_offset must be between -1.0 and 1.0");
        assert!(self.pha_ramp >= -1.0 && self.pha_ramp <= 1.0, "pha_ramp must be between -1.0 and 1.0");
        assert!(self.repeat_speed >= 0.0 && self.repeat_speed <= 1.0, "repeat_speed must be between 0.0 and 1.0");
        assert!(self.arp_speed >= 0.0 && self.arp_speed <= 1.0, "arp_speed must be between 0.0 and 1.0");
        assert!(self.arp_mod >= -1.0 && self.arp_mod <= 1.0, "arp_mod must be between -1.0 and 1.0");
    }

    pub fn mutate(&mut self) {
        let rng = &mut rand::weak_rng();

        fn mutate_f64(rng: &mut Rng, v: &mut f64, min: f64, max: f64) {
            if rand_bool(rng, 1, 1) {
                *v = (*v + rand_f64(rng, -0.05, 0.05)).min(max).max(min);
            }
        }
        fn mutate_f32(rng: &mut Rng, v: &mut f32, min: f32, max: f32) {
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

    pub fn pickup() -> Sample {
        let rng = &mut rand::weak_rng();
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

    pub fn laser() -> Sample {
        let rng = &mut rand::weak_rng();
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

    pub fn explosion() -> Sample {
        let rng = &mut rand::weak_rng();
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

    pub fn powerup() -> Sample {
        let rng = &mut rand::weak_rng();
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

    pub fn hit() -> Sample {
        let rng = &mut rand::weak_rng();
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

    pub fn jump() -> Sample {
        let rng = &mut rand::weak_rng();
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

    pub fn blip() -> Sample {
        let rng = &mut rand::weak_rng();
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

pub struct Generator {
    sample: Sample,

    pub volume: f32,
    oscillator: Oscillator,
    hlpf: HighLowPassFilter,
    envelope: Envelope,
    phaser: Phaser,
    rep_time: i32,
    rep_limit: i32,
}
impl Generator {
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

            let sample = self.oscillator.by_ref()
                .chain_filter(&mut self.envelope)
                .chain_filter(&mut self.hlpf)
                .chain_filter(&mut self.phaser)
                .take(8)
                .sum::<f32>() / 8.0;

            *buffer_value = (sample * self.volume).min(1.0).max(-1.0);
        });
    }
    pub fn reset(&mut self) {
        self.restart();
        self.envelope.reset(self.sample.env_attack, self.sample.env_sustain, self.sample.env_decay, self.sample.env_punch);
        self.phaser.reset(self.sample.pha_offset, self.sample.pha_ramp);

        self.oscillator.reset_phase();
        self.oscillator.reset_vibrato(self.sample.vib_speed, self.sample.vib_strength);
        self.oscillator.reset_noise();

        self.rep_time = 0;
        self.rep_limit = ((1.0 - self.sample.repeat_speed).powi(2) * 20_000.0 * 32.0) as i32;

        if self.sample.repeat_speed == 0.0 {
            self.rep_limit = 0;
        }
    }
    pub fn restart(&mut self) {
        self.hlpf.reset(self.sample.lpf_resonance, self.sample.lpf_freq, self.sample.lpf_ramp,
                        self.sample.hpf_freq, self.sample.hpf_ramp);
        self.oscillator.reset(self.sample.wave_type, self.sample.base_freq, self.sample.freq_limit,
                              self.sample.freq_ramp, self.sample.freq_dramp,
                              self.sample.duty, self.sample.duty_ramp,
                              self.sample.arp_speed, self.sample.arp_mod);
    }
}

fn rand_f32(rng: &mut Rng, from: f32, until: f32) -> f32 {
    from + (until - from) * rng.next_f32()
}
fn rand_f64(rng: &mut Rng, from: f64, until: f64) -> f64 {
    from + (until - from) * rng.next_f64()
}
fn rand_bool(rng: &mut Rng, chance_true: u32, chance_false: u32) -> bool {
    rng.next_u32() % (chance_true + chance_false) < chance_true
}
fn rand_element<T: Copy>(rng: &mut Rng, slice: &[T]) -> T {
    slice[rng.next_u32() as usize % slice.len()]
}
