use nih_plug::prelude::*;
use crate::effects::{gain, chorus, EffectType};

#[derive(Params)]
pub struct PluginParams {
    #[id = "effect_type"]
    pub effect_type: EnumParam<EffectType>,

    #[nested(group = "gain")]
    pub gain_params: gain::GainParams,

    #[nested(group = "chorus")]
    pub chorus_params: chorus::ChorusParams, // Neuer Chorus-Parameter
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            effect_type: EnumParam::new(
                "Effect Type",
                EffectType::Gain, // Standardmäßig Gain
            ),
            gain_params: gain::GainParams::default(),
            chorus_params: chorus::ChorusParams::default(), // Standard-Chorus-Parameter
        }
    }
}

impl PluginParams {
    pub fn get_params(&self, effect_index: usize) -> &dyn Params {
        match effect_index {
            0 => &self.gain_params,
            1 => &self.chorus_params, // Rückgabe der Chorus-Parameter
            _ => &self.gain_params,
        }
    }
}
