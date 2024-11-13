use nih_plug::prelude::*;
use crate::effects::{EffectType, effect1, effect2, AsAny};
use std::sync::Arc;

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
    pub fn active_params(&self) -> Arc<dyn Params> {
        match self.effect_type.value() {
            EffectType::Effect1 => Arc::new(self.effect1_params.clone()),
            EffectType::Effect2 => Arc::new(self.effect2_params.clone()),
        }
    }
}

impl AsAny for PluginParams {}
