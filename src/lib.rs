extern crate rand;
use rand::Rng;


#[derive(PartialEq)]
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
    phase: i32,
    fperiod: f64,
    fmaxperiod: f64,
    fslide: f64,
    fdslide: f64,
    square_duty: f32,
    square_slide: f32,
    envelope: Envelope,
    env_vol: f32,
    fphase: f32,
    fdphase: f32,
    phaser: Phaser,
    noise_buffer: [f32; 32],
    fltp: f32,
    fltdp: f32,
    fltw: f32,
    fltw_d: f32,
    fltdmp: f32,
    fltphp: f32,
    flthp: f32,
    flthp_d: f32,
    vib_phase: f64,
    vib_speed: f64,
    vib_amp: f64,
    rep_time: i32,
    rep_limit: i32,
    arp_time: i32,
    arp_limit: i32,
    arp_mod: f64,
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

struct Phaser {
    ipp: usize,
    buffer: [f32; 1024]
}

impl Generator {
    pub fn new(s: Sample) -> Generator {
        s.assert_valid();
        let mut g = Generator {
            sample: s,
            volume: 0.2,
            playing_sample: true,
            phase: 0,
            fperiod: 0.0,
            fmaxperiod: 0.0,
            fslide: 0.0,
            fdslide: 0.0,
            square_duty: 0.0,
            square_slide: 0.0,
            envelope: Envelope::new(),
            env_vol: 0.0,
            fphase: 0.0,
            fdphase: 0.0,
            phaser: Phaser::new(),
            noise_buffer: [0.0; 32],
            fltp: 0.0,
            fltdp: 0.0,
            fltw: 0.0,
            fltw_d: 0.0,
            fltdmp: 0.0,
            fltphp: 0.0,
            flthp: 0.0,
            flthp_d: 0.0,
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
        let mut rng = rand::weak_rng();
        buffer.iter_mut().for_each(|buffer_value| {
            if !self.playing_sample {
                return
            }
            self.rep_time += 1;

            if self.rep_limit != 0 && self.rep_time >= self.rep_limit {
                self.rep_time = 0;
                self.reset(true);
            }
            /*
               for(int i=0;i<length;i++)
               {
               if(!playing_sample)
               break;

               rep_time++;
               if(rep_limit!=0 && rep_time>=rep_limit)
               {
               rep_time=0;
               ResetSample(true);
               }
               */

            self.arp_time += 1;

            if self.arp_limit != 0 && self.arp_time >= self.arp_limit {
                self.arp_limit = 0;
                self.fperiod *= self.arp_mod as f64;
            }

            self.fslide += self.fdslide;
            self.fperiod = (self.fperiod * self.fslide).min(self.fmaxperiod);

            /*
            // frequency envelopes/arpeggios
            arp_time++;
            if(arp_limit!=0 && arp_time>=arp_limit)
            {
            arp_limit=0;
            fperiod*=arp_mod;
            }
            fslide+=fdslide;
            fperiod*=fslide;
            if(fperiod>fmaxperiod)
            {
            fperiod=fmaxperiod;
            if(p_freq_limit>0.0f)
            playing_sample=false;
            }
            */

            self.vib_phase += self.vib_speed;
            let vibrato = 1.0 + self.vib_phase.sin() * self.vib_amp;

            let period = ((vibrato * self.fperiod) as i32).max(8);

            /*
               float rfperiod=fperiod;
               if(vib_amp>0.0f)
               {
               vib_phase+=vib_speed;
               rfperiod=fperiod*(1.0+sin(vib_phase)*vib_amp);
               }
               period=(int)rfperiod;
               if(period<8) period=8;
               */
            self.square_duty = (self.square_duty + self.square_slide).min(0.5).max(0.0);

            /*
               square_duty+=square_slide;
               if(square_duty<0.0f) square_duty=0.0f;
               if(square_duty>0.5f) square_duty=0.5f;
               */

            self.envelope.advance();
            self.env_vol = self.envelope.volume();

            /*
            // volume envelope
            env_time++;
            if(env_time>env_length[env_stage])
            {
            env_time=0;
            env_stage++;
            if(env_stage==3)
            playing_sample=false;
            }
            if(env_stage==0)
            env_vol=(float)env_time/env_length[0];
            if(env_stage==1)
            env_vol=1.0f+pow(1.0f-(float)env_time/env_length[1], 1.0f)*2.0f*p_env_punch;
            if(env_stage==2)
            env_vol=1.0f-(float)env_time/env_length[2];

*/
            self.fphase += self.fdphase;

            if self.flthp_d != 0.0 {
                self.flthp = (self.flthp * self.flthp_d).min(0.1).max(0.00001);
            }
            /*
            // phaser step
            fphase+=fdphase;
            iphase=abs((int)fphase);
            if(iphase>1023) iphase=1023;

            if(flthp_d!=0.0f)
            {
            flthp*=flthp_d;
            if(flthp<0.00001f) flthp=0.00001f;
            if(flthp>0.1f) flthp=0.1f;
            }
            */

            let mut ssample = 0.0;

            for _ in 0..8 {
                let mut sample;
                self.phase += 1;
                if self.phase >= period {
                    self.phase = self.phase % period;
                    if self.sample.wave_type == WaveType::Noise {
                        self.noise_buffer.iter_mut()
                            .for_each(|v| *v = rng.next_f32() * 2.0 - 1.0);
                    }
                }

                /*
                   float ssample=0.0f;
                   for(int si=0;si<8;si++) // 8x supersampling
                   {
                   float sample=0.0f;
                   phase++;
                   if(phase>=period)
                   {
                //				phase=0;
                phase%=period;
                if(wave_type==3)
                for(int i=0;i<32;i++)
                noise_buffer[i]=frnd(2.0f)-1.0f;
                }
                */
                let fp = self.phase as f32 / period as f32;
                sample = match self.sample.wave_type {
                    WaveType::Square => if fp < self.square_duty { 0.5 } else { -0.5 },
                    WaveType::Triangle => 1.0 - fp * 2.0,
                    WaveType::Sine => (fp * 2.0 * PI).sin(),
                    WaveType::Noise => self.noise_buffer[(fp * 32.0) as usize]
                };
                /*
                // base waveform
                float fp=(float)phase/period;
                switch(wave_type)
                {
                case 0: // square
                if(fp<square_duty)
                sample=0.5f;
                else
                sample=-0.5f;
                break;
                case 1: // sawtooth
                sample=1.0f-fp*2;
                break;
                case 2: // sine
                sample=(float)sin(fp*2*PI);
                break;
                case 3: // noise
                sample=noise_buffer[phase*32/period];
                break;
                }
                */

                sample = {
                    // Low pass filter
                    let pp = self.fltp;

                    self.fltw = (self.fltw * self.fltw_d).min(0.1).max(0.0);

                    if self.sample.lpf_freq < 1.0 {
                        self.fltdp += (sample - self.fltp) * self.fltw;
                        self.fltdp -= self.fltdp * self.fltdmp;
                    } else {
                        self.fltp = sample;
                        self.fltdp = 0.0;
                    }

                    self.fltp += self.fltdp;
                    /*
                    // lp filter
                    float pp=fltp;
                    fltw*=fltw_d;
                    if(fltw<0.0f) fltw=0.0f;
                    if(fltw>0.1f) fltw=0.1f;
                    if(p_lpf_freq!=1.0f)
                    {
                    fltdp+=(sample-fltp)*fltw;
                    fltdp-=fltdp*fltdmp;
                    }
                    else
                    {
                    fltp=sample;
                    fltdp=0.0f;
                    }
                    fltp+=fltdp;
                    */

                    // High pass filter
                    self.fltphp += self.fltp - pp;
                    self.fltphp -= self.fltphp * self.flthp;

                    /*
                    // hp filter
                    fltphp+=fltp-pp;
                    fltphp-=fltphp*flthp;
                    sample=fltphp;
                    */
                    self.fltphp
                };

                sample = self.phaser.phase(self.fphase, sample);

                ssample += sample * self.env_vol;

                /*
                // phaser
                phaser_buffer[ipp&1023]=sample;
                sample+=phaser_buffer[(ipp-iphase+1024)&1023];
                ipp=(ipp+1)&1023;
                // final accumulation and envelope application
                ssample+=sample*env_vol;
                }
                */
            }

            // Average supersamples, apply volume, limit to [-1.0..1.0]
            *buffer_value = (ssample * self.volume / 8.0).min(1.0).max(-1.0);
            /*
               ssample=ssample/8*master_vol;

               ssample*=2.0f*sound_vol;

               if(buffer!=NULL)
               {
               if(ssample>1.0f) ssample=1.0f;
               if(ssample<-1.0f) ssample=-1.0f;
             *buffer++=ssample;
             }
             */
        });
    }
    pub fn reset(&mut self, restart: bool) {
        if !restart {
            self.phase = 0;
        }

        self.fperiod = 100.0 / ((self.sample.base_freq as f64).powi(2) + 0.001);
        self.fmaxperiod = 100.0 / ((self.sample.freq_limit as f64).powi(2) + 0.001);
        self.fslide = 1.0 - (self.sample.freq_ramp as f64).powi(3) * 0.01;
        self.fdslide = -(self.sample.freq_dramp as f64).powi(3) * 0.000001;
        self.square_duty = 0.5 - self.sample.duty * 0.5;
        self.square_slide = -self.sample.duty_ramp * 0.00005;
        /*
           if(!restart)
           phase=0;
           fperiod=100.0/(p_base_freq*p_base_freq+0.001);
           period=(int)fperiod;
           fmaxperiod=100.0/(p_freq_limit*p_freq_limit+0.001);
           fslide=1.0-pow((double)p_freq_ramp, 3.0)*0.01;
           fdslide=-pow((double)p_freq_dramp, 3.0)*0.000001;
           square_duty=0.5f-p_duty*0.5f;
           square_slide=-p_duty_ramp*0.00005f;
           */

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
        /*
           if(p_arp_mod>=0.0f)
           arp_mod=1.0-pow((double)p_arp_mod, 2.0)*0.9;
           else
           arp_mod=1.0+pow((double)p_arp_mod, 2.0)*10.0;
           arp_time=0;
           arp_limit=(int)(pow(1.0f-p_arp_speed, 2.0f)*20000+32);
           if(p_arp_speed==1.0f)
           arp_limit=0;
           */

        if !restart {
            self.fltp = 0.0;
            self.fltdp = 0.0;
            self.fltw = self.sample.lpf_freq.powi(3) * 0.1;
            self.fltw_d = 1.0 + self.sample.lpf_ramp * 0.0001;
            /*
               if(!restart)
               {
            // reset filter
            fltp=0.0f;
            fltdp=0.0f;
            fltw=pow(p_lpf_freq, 3.0f)*0.1f;
            fltw_d=1.0f+p_lpf_ramp*0.0001f;
            */

            self.fltdmp = 5.0 / (1.0 + self.sample.lpf_resonance.powi(2) * 20.0) * (0.01 + self.fltw);
            if self.fltdmp > 0.8 {
                self.fltdmp = 0.8;
            }

            self.fltphp = 0.0;
            self.flthp = self.sample.hpf_freq.powi(2) * 0.1;
            self.flthp_d = 1.0 + self.sample.hpf_ramp * 0.0003;
            /*
               fltdmp=5.0f/(1.0f+pow(p_lpf_resonance, 2.0f)*20.0f)*(0.01f+fltw);
               if(fltdmp>0.8f) fltdmp=0.8f;
               fltphp=0.0f;
               flthp=pow(p_hpf_freq, 2.0f)*0.1f;
               flthp_d=1.0+p_hpf_ramp*0.0003f;
               */

            self.vib_phase = 0.0;
            self.vib_speed = self.sample.vib_speed.powi(2) * 0.01;
            self.vib_amp = self.sample.vib_strength * 0.5;
            /*
            // reset vibrato
            vib_phase=0.0f;
            vib_speed=pow(p_vib_speed, 2.0f)*0.01f;
            vib_amp=p_vib_strength*0.5f;
            */

            self.env_vol = 0.0;

            let attack = (self.sample.env_attack.powi(2) * 100_000.0) as u32;
            let sustain = (self.sample.env_sustain.powi(2) * 100_000.0) as u32;
            let decay = (self.sample.env_decay.powi(2) * 100_000.0) as u32;
            self.envelope.reset(attack, sustain, decay, self.sample.env_punch);
            /*
            // reset envelope
            env_vol=0.0f;
            env_stage=0;
            env_time=0;
            env_length[0]=(int)(p_env_attack*p_env_attack*100000.0f);
            env_length[1]=(int)(p_env_sustain*p_env_sustain*100000.0f);
            env_length[2]=(int)(p_env_decay*p_env_decay*100000.0f);
            */

            self.fphase == self.sample.pha_offset.powi(2) * 1020.0;

            if self.sample.pha_offset < 0.0 {
                self.fphase = -self.fphase
            }

            self.fdphase = self.sample.pha_ramp.powi(2) * 1.0;

            if self.sample.pha_ramp < 0.0 {
                self.fdphase = -self.fdphase;
            }

            /*
               fphase=pow(p_pha_offset, 2.0f)*1020.0f;
               if(p_pha_offset<0.0f) fphase=-fphase;
               fdphase=pow(p_pha_ramp, 2.0f)*1.0f;
               if(p_pha_ramp<0.0f) fdphase=-fdphase;
               iphase=abs((int)fphase);
               ipp=0;
               */

            self.phaser = Phaser::new();
            let mut rng = rand::weak_rng();
            self.noise_buffer.iter_mut().for_each(|v| {
                *v = rng.next_f32() * 2.0 - 1.0;
            });

            self.rep_time = 0;
            self.rep_limit = ((1.0 - self.sample.repeat_speed).powi(2) * 20_000.0 * 32.0) as i32;

            if self.sample.repeat_speed == 0.0 {
                self.rep_limit = 0;
            }
            /*
               for(int i=0;i<1024;i++)
               phaser_buffer[i]=0.0f;

               for(int i=0;i<32;i++)
               noise_buffer[i]=frnd(2.0f)-1.0f;

               rep_time=0;
               rep_limit=(int)(pow(1.0f-p_repeat_speed, 2.0f)*20000+32);
               if(p_repeat_speed==0.0f)
               rep_limit=0;
               }
               */
        }
    }
}

const PI: f32 = 3.14159265359;

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

impl Phaser {
    fn new() -> Phaser {
        Phaser {
            ipp: 0,
            buffer: [0.0; 1024]
        }
    }
    fn phase(&mut self, fphase: f32, sample: f32) -> f32 {
        let p_len = self.buffer.len();
        self.buffer[self.ipp % p_len] = sample;
        let iphase = (fphase.abs() as i32).min(p_len as i32 - 1);
        let result = sample + self.buffer[(self.ipp + p_len - iphase as usize) % p_len];
        self.ipp = (self.ipp + 1) % p_len;
        result
    }

}
