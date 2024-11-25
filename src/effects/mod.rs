use nih_plug::prelude::*;
use std::any::Any;
use nih_plug::prelude::Enum;

pub mod effect1;
pub mod effect2;

pub trait Effect: Send {
    fn process(&self, samples: &mut [f32], sample_rate: f32, params: &dyn Params);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EffectType {
    Effect1,
    Effect2,
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Effect1
    }
}

// Implementierung des Enum-Traits für EffectType
impl Enum for EffectType {
    fn variants() -> &'static [&'static str] {
        &["Effect1", "Effect2"]
    }

    fn ids() -> Option<&'static [&'static str]> {
        Some(&["effect1", "effect2"])
    }

    fn to_index(self) -> usize {
        self as usize
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => EffectType::Effect1,
            1 => EffectType::Effect2,
            _ => EffectType::Effect1, // Fallback
        }
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
