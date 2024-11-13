use crate::effects::Effect;
use nih_plug::prelude::*;
use std::sync::Arc;
use std::any::Any;

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

impl Clone for Effect1Params {
    fn clone(&self) -> Self {
        Self {
            gain: self.gain.clone(),
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
    fn process(&self, samples: &mut [f32], _sample_rate: f32, params: &dyn Params) {
        let gain = if let Some(p) = params.as_any().downcast_ref::<Effect1Params>() {
            p.gain.value()
        } else {
            1.0 // Default value if downcast fails
        };
        for sample in samples {
            *sample *= gain;
        }
    }
}
