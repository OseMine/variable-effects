use nih_plug::prelude::*;
pub mod gain;
pub mod chorus;

pub trait Effect: Send {
    fn process(&mut self, samples: &mut [f32], sample_rate: f32, params: &dyn Params);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EffectType {
    Gain,
    Chorus,
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Gain
    }
}

impl Enum for EffectType {
    fn variants() -> &'static [&'static str] {
        &["Gain", "Chorus"]
    }
    fn ids() -> Option<&'static [&'static str]> {
        Some(&["gain", "chorus"])
    }
    fn to_index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Self {
        match index {
            0 => EffectType::Gain,
            1 => EffectType::Chorus,
            _ => EffectType::Gain,
        }
    }
}