use nih_plug::prelude::*;
use std::sync::Arc;
use crate::effects::Effect;

#[derive(Params)]
pub struct Effect2Params {
    #[id = "gainnnnnn"]
    pub gain: FloatParam,
}

impl Default for Effect2Params {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gainnnnnnnn",
                1.0,
                FloatRange::Linear { min: 0.0, max: 2.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_step_size(0.01),
        }
    }
}

pub struct Effect2 {
    params: Arc<Effect2Params>,
}

impl Effect2 {
    pub fn new() -> Self {
        Self {
            params: Arc::new(Effect2Params::default()),
        }
    }
}

impl Effect for Effect2 {
    fn process(&self, samples: &mut [f32], _sample_rate: f32, _params: &dyn Params) {
        // Beispiel: einfacher Gain-Prozess
        for sample in samples.iter_mut() {
            *sample *= self.params.gain.value(); // Anwendung des Gain-Werts
        }
    }
}
