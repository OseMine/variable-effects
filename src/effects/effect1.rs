use nih_plug::prelude::*;
use std::sync::Arc;
use crate::effects::Effect;

#[derive(Params)]
pub struct Effect1Params {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for Effect1Params {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 2.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_step_size(0.01),
        }
    }
}

pub struct Effect1 {
    params: Arc<Effect1Params>,
}

impl Effect1 {
    pub fn new() -> Self {
        Self {
            params: Arc::new(Effect1Params::default()),
        }
    }
}

impl Effect for Effect1 {
    fn process(&self, samples: &mut [f32], _sample_rate: f32, _params: &dyn Params) {
        // Beispiel: einfacher Gain-Prozess
        for sample in samples.iter_mut() {
            *sample *= self.params.gain.value(); // Anwendung des Gain-Werts
        }
    }
}
