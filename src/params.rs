use nih_plug::prelude::*;
use crate::effects::{EffectType, effect1, effect2};

#[derive(Params)]
pub struct PluginParams {
    #[id = "effect_type"]
    pub effect_type: EnumParam<EffectType>,

    #[nested(group = "Effect1")]
    pub effect1_params: effect1::Effect1Params,

    #[nested(group = "Effect2")]
    pub effect2_params: effect2::Effect2Params,
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            effect_type: EnumParam::new(
                "Effect Type",
                EffectType::Effect1,
            ),
            effect1_params: effect1::Effect1Params::default(),
            effect2_params: effect2::Effect2Params::default(),
        }
    }
}

impl PluginParams {
    // Diese Methode gibt die entsprechenden Parameter basierend auf dem Effektindex zurück
    pub fn get_params(&self, effect_index: usize) -> &dyn Params {
        match effect_index {
            0 => &self.effect1_params,
            1 => &self.effect2_params,
            _ => &self.effect1_params, // Fallback zu Effect1 bei ungültigem Index
        }
    }
}
