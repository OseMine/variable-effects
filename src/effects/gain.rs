use nih_plug::prelude::*;
use crate::Effect;
use std::sync::Arc;

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
                0.0,
                FloatRange::Linear { min: -60.0, max: 12.0 }, // -60 dB bis +12 dB
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)), // Glättung über 50ms
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
        // Den aktuellen Gain-Wert in Dezibel holen
        let gain_db = self.params.gain.value();
        
        // Gain-Wert in linearen Verstärkungsfaktor umrechnen
        let gain_factor = 10f32.powf(gain_db / 20.0);

        // Audio-Daten verarbeiten
        for sample in samples.iter_mut() {
            *sample *= gain_factor;
        }
    }
}
