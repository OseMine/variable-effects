use nih_plug::prelude::*;
use std::sync::Arc;
use crate::Effect;

pub struct Gain {
    params: Arc<GainParams>,
}

#[derive(Params)]
pub struct GainParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for GainParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Gain {
    pub fn new() -> Self {
        Self {
            params: Arc::new(GainParams::default()),
        }
    }
}

impl Effect for Gain {
    fn process(&mut self, samples: &mut [f32], _sample_rate: f32, _params: &dyn Params) {
        let gain = self.params.gain.smoothed.next();
        for sample in samples.iter_mut() {
            *sample *= gain;
        }
    }
}