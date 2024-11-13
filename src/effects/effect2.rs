use crate::effects::Effect;
use nih_plug::prelude::*;
use std::sync::Arc;
use std::any::Any;

#[derive(Params)]
pub struct Effect2Params {
    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for Effect2Params {
    fn default() -> Self {
        Self {
            mix: FloatParam::new(
                "Mix",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_step_size(0.01),
        }
    }
}

impl Clone for Effect2Params {
    fn clone(&self) -> Self {
        Self {
            mix: self.mix.clone(),
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
    fn process(&self, samples: &mut [f32], _sample_rate: f32, params: &dyn Params) {
        let mix = if let Some(p) = params.as_any().downcast_ref::<Effect2Params>() {
            p.mix.value()
        } else {
            0.5 // Default value if downcast fails
        };
        for sample in samples {
            *sample = *sample * mix + *sample * (1.0 - mix);
        }
    }
}
