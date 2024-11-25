use nih_plug::prelude::*;
use std::sync::Arc;
use std::f32::consts::PI;

#[derive(Clone)]
struct LFO {
    phase: f32,
}

impl LFO {
    fn new() -> Self {
        Self { phase: 0.0 }
    }
    
    fn reset(&mut self) {
        self.phase = 0.0;
    }
    
    fn next(&mut self, rate: f32, sample_rate: f32) -> f32 {
        self.phase = (self.phase + rate / sample_rate) % 1.0;
        (2.0 * PI * self.phase).sin()
    }
}

#[derive(Clone)]
struct DelayBuffer {
    buffer: Vec<f32>,
    write_pos: usize,
}

impl DelayBuffer {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            write_pos: 0,
        }
    }
    
    fn resize(&mut self, size: usize) {
        self.buffer.resize(size, 0.0);
        self.write_pos = 0;
    }
    
    fn write(&mut self, sample: f32) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
    }
    
    fn read(&self, delay_samples: f32) -> f32 {
        let delay_samples = delay_samples as usize;
        let read_pos = (self.write_pos + self.buffer.len() - delay_samples) % self.buffer.len();
        self.buffer[read_pos]
    }
}

#[derive(Params, Clone)]
pub struct ChorusParams {
    #[id = "dry_wet"]
    pub dry_wet: FloatParam,
    #[id = "depth"]
    pub depth: FloatParam,
    #[id = "rate"]
    pub rate: FloatParam,
    #[id = "voices"]
    pub voices: IntParam,
    #[id = "delay_ms"]
    pub delay_ms: FloatParam,
    #[id = "stereo_width"]
    pub stereo_width: FloatParam,
}

impl Default for ChorusParams {
    fn default() -> Self {
        Self {
            dry_wet: FloatParam::new(
                "Dry/Wet",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            depth: FloatParam::new(
                "Depth",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            rate: FloatParam::new(
                "Rate",
                1.0,
                FloatRange::Linear { min: 0.1, max: 10.0 },
            )
            .with_unit(" Hz"),
            voices: IntParam::new("Voices", 2, IntRange::Linear { min: 1, max: 6 }),
            delay_ms: FloatParam::new(
                "Delay",
                20.0,
                FloatRange::Linear { min: 1.0, max: 50.0 },
            )
            .with_unit(" ms"),
            stereo_width: FloatParam::new(
                "Stereo Width",
                0.8,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

pub struct Chorus {
    params: Arc<ChorusParams>,
    delay: DelayBuffer,
    lfos: Vec<LFO>,
    current_sample_rate: f32,
}

impl Chorus {
    pub fn new() -> Self {
        Self {
            params: Arc::new(ChorusParams::default()),
            delay: DelayBuffer::new(48000),
            lfos: vec![LFO::new(); 6],
            current_sample_rate: 44100.0,
        }
    }

    pub fn initialize_with_sample_rate(&mut self, sample_rate: f32) {
        self.current_sample_rate = sample_rate;
        self.delay.resize((sample_rate * 0.1) as usize);
        self.lfos.iter_mut().for_each(|lfo| lfo.reset());
    }

    fn internal_process(&mut self, samples: &mut [f32], sample_rate: f32) {
        let voices = self.params.voices.value() as usize;
        let rate = self.params.rate.value();
        let depth = self.params.depth.value();
        let dry_wet = self.params.dry_wet.value();
        let delay_ms = self.params.delay_ms.value();
        let stereo_width = self.params.stereo_width.value();
        
        let base_delay_samples = (delay_ms * 0.001 * sample_rate) as f32;
        let num_channels = 2;
        let frames = samples.len() / num_channels;
        let mut output_buffer = vec![0.0; samples.len()];

        for frame in 0..frames {
            let left_idx = frame * 2;
            let right_idx = frame * 2 + 1;
            let input_sample = (samples[left_idx] + samples[right_idx]) * 0.5;
            
            self.delay.write(input_sample);
            
            let mut chorus_out_l = 0.0;
            let mut chorus_out_r = 0.0;

            for voice in 0..voices {
                let lfo = &mut self.lfos[voice];
                let mod_delay = base_delay_samples * (1.0 + depth * lfo.next(rate, sample_rate));
                let delayed_sample = self.delay.read(mod_delay);
                
                let stereo_pos = if voices > 1 {
                    (voice as f32 / (voices - 1) as f32) * 2.0 - 1.0
                } else {
                    0.0
                };
                
                let stereo_mod = stereo_pos * stereo_width;
                let pan_left = (1.0 - stereo_mod) * 0.5;
                let pan_right = (1.0 + stereo_mod) * 0.5;
                
                chorus_out_l += delayed_sample * pan_left;
                chorus_out_r += delayed_sample * pan_right;
            }

            if voices > 0 {
                chorus_out_l /= voices as f32;
                chorus_out_r /= voices as f32;
            }

            output_buffer[left_idx] = samples[left_idx] * (1.0 - dry_wet) + chorus_out_l * dry_wet;
            output_buffer[right_idx] = samples[right_idx] * (1.0 - dry_wet) + chorus_out_r * dry_wet;
        }

        samples.copy_from_slice(&output_buffer);
    }
}

impl Effect for Chorus {
    fn process(&mut self, samples: &mut [f32], sample_rate: f32, params: &dyn Params) {
        if (self.current_sample_rate - sample_rate).abs() > std::f32::EPSILON {
            self.initialize_with_sample_rate(sample_rate);
        }

        if let Some(chorus_params) = params.downcast_ref::<ChorusParams>() {
            self.params = Arc::new(chorus_params.clone());
        }

        self.internal_process(samples, sample_rate);
    }
}