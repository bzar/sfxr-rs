extern crate rand;
use rand::{Rng, XorShiftRng};


#[derive(PartialEq,Copy,Clone)]
pub enum WaveType { Square, Triangle, Sine, Noise }
pub struct Sample {
    wave_type: WaveType,
    pub base_freq: f32,
    pub freq_limit: f32,
    pub freq_ramp: f32,
    pub freq_dramp: f32,
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
    pub arp_mod: f32
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
}

pub struct Generator {
    sample: Sample,

    pub volume: f32,
    playing_sample: bool,
    oscillator: Oscillator,
    hlpf: HighLowPassFilter,
    fperiod: f64,
    fmaxperiod: f64,
    fslide: f64,
    fdslide: f64,
    square_slide: f32,
    envelope: Envelope,
    phaser: Phaser,
    vib_phase: f64,
    vib_speed: f64,
    vib_amp: f64,
    rep_time: i32,
    rep_limit: i32,
    arp_time: i32,
    arp_limit: i32,
    arp_mod: f64,
}

struct Oscillator {
    wave_type: WaveType,
    square_duty: f32,
    period: u32,
    phase: u32,
    noise_buffer: [f32; 32],
    rng:XorShiftRng
}

enum EnvelopeStage { Attack, Sustain, Decay, End }
struct Envelope {
    stage: EnvelopeStage,
    stage_left: u32,
    attack: u32,
    sustain: u32,
    decay: u32,
    punch: f32
}

struct HighLowPassFilter {
    fltp: f32,
    fltdp: f32,
    fltw: f32,
    fltw_d: f32,
    fltdmp: f32,
    fltphp: f32,
    flthp: f32,
    flthp_d: f32,
}

struct Phaser {
    ipp: usize,
    fphase: f32,
    fdphase: f32,
    buffer: [f32; 1024]
}

impl Generator {
    pub fn new(s: Sample) -> Generator {
        s.assert_valid();
        let wave_type = s.wave_type;
        let mut g = Generator {
            sample: s,
            volume: 0.2,
            playing_sample: true,
            oscillator: Oscillator::new(wave_type),
            hlpf: HighLowPassFilter::new(),
            fperiod: 0.0,
            fmaxperiod: 0.0,
            fslide: 0.0,
            fdslide: 0.0,
            square_slide: 0.0,
            envelope: Envelope::new(),
            phaser: Phaser::new(),
            vib_phase: 0.0,
            vib_speed: 0.0,
            vib_amp: 0.0,
            rep_time: 0,
            rep_limit: 0,
            arp_time: 0,
            arp_limit: 0,
            arp_mod: 0.0,
        };

        g.reset(false);

        g
    }
    pub fn generate(&mut self, buffer: &mut [f32]) {
        buffer.iter_mut().for_each(|buffer_value| {
            if !self.playing_sample {
                return
            }
            self.rep_time += 1;

            if self.rep_limit != 0 && self.rep_time >= self.rep_limit {
                self.rep_time = 0;
                self.reset(true);
            }

            self.arp_time += 1;

            if self.arp_limit != 0 && self.arp_time >= self.arp_limit {
                self.arp_limit = 0;
                self.fperiod *= self.arp_mod as f64;
            }

            self.fslide += self.fdslide;
            self.fperiod = (self.fperiod * self.fslide).min(self.fmaxperiod);

            self.vib_phase += self.vib_speed;
            let vibrato = 1.0 + self.vib_phase.sin() * self.vib_amp;

            self.oscillator.period = ((vibrato * self.fperiod) as u32).max(8);
            self.oscillator.square_slide(self.square_slide);

            self.envelope.advance();
            let env_vol = self.envelope.volume();

            self.phaser.advance();

            // Need to borrow separately to appease borrow checker
            let oscillator = &mut self.oscillator;
            let hlpf = &mut self.hlpf;
            let phaser = &mut self.phaser;
            let sample = (0..8)
                .map(|_| oscillator.next().unwrap() * env_vol)
                .map(|s| hlpf.filter(s))
                .map(|s| phaser.phase(s))
                .sum::<f32>() / 8.0;

            *buffer_value = (sample * self.volume).min(1.0).max(-1.0);
        });
    }
    pub fn reset(&mut self, restart: bool) {
        if !restart {
            self.oscillator.phase = 0;
        }

        self.hlpf.reset(self.sample.lpf_resonance, self.sample.lpf_freq, self.sample.lpf_ramp,
                        self.sample.hpf_freq, self.sample.hpf_ramp);
        self.fperiod = 100.0 / ((self.sample.base_freq as f64).powi(2) + 0.001);
        self.fmaxperiod = 100.0 / ((self.sample.freq_limit as f64).powi(2) + 0.001);
        self.fslide = 1.0 - (self.sample.freq_ramp as f64).powi(3) * 0.01;
        self.fdslide = -(self.sample.freq_dramp as f64).powi(3) * 0.000001;
        self.oscillator.wave_type = self.sample.wave_type;
        self.oscillator.square_duty = 0.5 - self.sample.duty * 0.5;
        self.square_slide = -self.sample.duty_ramp * 0.00005;

        self.arp_mod = if self.sample.arp_mod >= 0.0 {
            1.0 - (self.sample.arp_mod as f64).powf(2.0) * 0.9
        } else {
            1.0 - (self.sample.arp_mod as f64).powf(2.0) * 10.0
        };

        self.arp_time = 0;
        self.arp_limit = ((1.0 - self.sample.arp_speed).powi(2) * 20000.0 + 32.0) as i32;

        if self.sample.arp_speed == 1.0 {
            self.arp_limit = 0;
        }

        if !restart {

            self.vib_phase = 0.0;
            self.vib_speed = self.sample.vib_speed.powi(2) * 0.01;
            self.vib_amp = self.sample.vib_strength * 0.5;

            let attack = (self.sample.env_attack.powi(2) * 100_000.0) as u32;
            let sustain = (self.sample.env_sustain.powi(2) * 100_000.0) as u32;
            let decay = (self.sample.env_decay.powi(2) * 100_000.0) as u32;
            self.envelope.reset(attack, sustain, decay, self.sample.env_punch);
            self.phaser.reset(self.sample.pha_offset, self.sample.pha_ramp);
            self.oscillator.reset_noise();

            self.rep_time = 0;
            self.rep_limit = ((1.0 - self.sample.repeat_speed).powi(2) * 20_000.0 * 32.0) as i32;

            if self.sample.repeat_speed == 0.0 {
                self.rep_limit = 0;
            }
        }
    }
}

const PI: f32 = 3.14159265359;

impl Oscillator {
    fn new(wave_type: WaveType) -> Oscillator {
        Oscillator  {
            wave_type,
            square_duty: 0.5,
            period: 8,
            phase: 0,
            noise_buffer: [0.0; 32],
            rng: rand::weak_rng()
        }
    }
    fn reset_noise(&mut self) {
        for v in self.noise_buffer.iter_mut() {
            *v = self.rng.next_f32() * 2.0 - 1.0;
        }
    }
    fn square_slide(&mut self, amount: f32) {
        self.square_duty = (self.square_duty + amount).min(0.5).max(0.0);
    }
}
impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.phase += 1;
        if self.phase >= self.period {
            self.phase = self.phase % self.period;
            if self.wave_type == WaveType::Noise {
                self.reset_noise();
            }
        }

        let fp = self.phase as f32 / self.period as f32;
        let sample = match self.wave_type {
            WaveType::Square => if fp < self.square_duty { 0.5 } else { -0.5 },
            WaveType::Triangle => 1.0 - fp * 2.0,
            WaveType::Sine => (fp * 2.0 * PI).sin(),
            WaveType::Noise => self.noise_buffer[(fp * 32.0) as usize]
        };

        Some(sample)
    }
}
impl Envelope {
    fn new() -> Envelope {
        Envelope {
            stage: EnvelopeStage::Attack,
            stage_left: 0,
            attack: 0,
            sustain: 0,
            decay: 0,
            punch: 0.0
        }
    }
    fn reset(&mut self, attack: u32, sustain: u32, decay: u32, punch: f32) {
        self.attack = attack;
        self.sustain = sustain;
        self.decay = decay;
        self.punch = punch;
        self.stage = EnvelopeStage::Attack;
        self.stage_left = self.current_stage_length();
    }
    fn advance(&mut self) {
        if self.stage_left > 1 {
            self.stage_left -= 1;
        } else {
            self.stage = match self.stage {
                EnvelopeStage::Attack => EnvelopeStage::Sustain,
                EnvelopeStage::Sustain => EnvelopeStage::Decay,
                EnvelopeStage::Decay => EnvelopeStage::End,
                EnvelopeStage::End => EnvelopeStage:: End
            };

            self.stage_left = self.current_stage_length();
        }
    }
    fn current_stage_length(&self) -> u32 {
        match self.stage {
            EnvelopeStage::Attack => self.attack,
            EnvelopeStage::Sustain => self.sustain,
            EnvelopeStage::Decay => self.decay,
            EnvelopeStage::End => 0
        }
    }
    fn volume(&self) -> f32 {
        let dt = self.stage_left as f32 / self.current_stage_length() as f32;
        match self.stage {
            EnvelopeStage::Attack => 1.0 - dt,
            EnvelopeStage::Sustain => 1.0 + dt * 2.0 * self.punch,
            EnvelopeStage::Decay => dt,
            EnvelopeStage::End => 0.0
        }
    }
}

impl HighLowPassFilter {
    fn new() -> HighLowPassFilter {
        HighLowPassFilter {
            fltp: 0.0,
            fltdp: 0.0,
            fltw: 0.0,
            fltw_d: 0.0,
            fltdmp: 0.0,
            fltphp: 0.0,
            flthp: 0.0,
            flthp_d: 0.0
        }
    }
    fn reset(&mut self, lpf_resonance: f32, lpf_freq: f32, lpf_ramp: f32, hpf_freq: f32, hpf_ramp: f32) {
        self.fltp = 0.0;
        self.fltdp = 0.0;
        self.fltw = lpf_freq.powi(3) * 0.1;
        self.fltw_d = 1.0 + lpf_ramp * 0.0001;

        self.fltdmp = 5.0 / (1.0 + lpf_resonance.powi(2) * 20.0) * (0.01 + self.fltw);
        if self.fltdmp > 0.8 {
            self.fltdmp = 0.8;
        }

        self.fltphp = 0.0;
        self.flthp = hpf_freq.powi(2) * 0.1;
        self.flthp_d = 1.0 + hpf_ramp * 0.0003;
    }
    fn filter(&mut self, sample: f32) -> f32 {
        let pp = self.fltp;

        if self.fltw > 0.0 {
            self.fltw = (self.fltw * self.fltw_d).min(0.1).max(0.0);
            self.fltdp += (sample - self.fltp) * self.fltw;
            self.fltdp -= self.fltdp * self.fltdmp;
        } else {
            self.fltp = sample;
            self.fltdp = 0.0;
        }

        self.fltp += self.fltdp;

        // High pass filter
        self.flthp = (self.flthp * self.flthp_d).min(0.1).max(0.00001);
        self.fltphp += self.fltp - pp;
        self.fltphp -= self.fltphp * self.flthp;

        self.fltphp
    }
}
impl Phaser {
    fn new() -> Phaser {
        Phaser {
            ipp: 0,
            fphase: 0.0,
            fdphase: 0.0,
            buffer: [0.0; 1024]
        }
    }
    fn reset(&mut self, pha_offset: f32, pha_ramp: f32) {
        self.fphase = pha_offset.powi(2) * 1020.0;

        if pha_offset < 0.0 {
            self.fphase = -self.fphase
        }

        self.fdphase = pha_ramp.powi(2) * 1.0;

        if pha_ramp < 0.0 {
            self.fdphase = -self.fdphase;
        }
    }

    fn advance(&mut self) {
        self.fphase += self.fdphase;
    }
    fn phase(&mut self, sample: f32) -> f32 {
        let p_len = self.buffer.len();
        self.buffer[self.ipp % p_len] = sample;
        let iphase = (self.fphase.abs() as i32).min(p_len as i32 - 1);
        let result = sample + self.buffer[(self.ipp + p_len - iphase as usize) % p_len];
        self.ipp = (self.ipp + 1) % p_len;
        result
    }

}
