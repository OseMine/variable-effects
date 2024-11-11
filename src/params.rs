use nih_plug::prelude::*;
use crate::effects::EffectType;

#[derive(Params)]
pub struct PluginParams {
    #[id = "effect_type"]
    pub effect_type: EnumParam<EffectType>,

    // Effect 1 Parameter
    #[id = "effect1_gain"]
    pub effect1_gain: FloatParam,

    // Effect 2 Parameter
    #[id = "effect2_frequency"]
    pub effect2_frequency: FloatParam,
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            effect_type: EnumParam::new(
                "Effect Type",
                EffectType::Effect1,
            ),
            effect1_gain: FloatParam::new(
                "Effect 1 Gain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 2.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_step_size(0.01),
            effect2_frequency: FloatParam::new(
                "Effect 2 Frequency",
                440.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: 0.5
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_step_size(1.0)
            .with_unit(" Hz"),
        }
    }
}
