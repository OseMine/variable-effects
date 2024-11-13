use nih_plug::prelude::*;
use std::any::Any;
pub mod effect1;
pub mod effect2;

/// Das allgemeine Effekt-Interface, das für alle Effekte implementiert wird.
pub trait Effect: Send {
    fn process(&self, samples: &mut [f32], sample_rate: f32, params: &dyn Params);
}

/// Enum zur Auswahl der Effekte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum EffectType {
    Effect1,
    Effect2,
}

impl std::fmt::Display for EffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectType::Effect1 => write!(f, "Effect 1"),
            EffectType::Effect2 => write!(f, "Effect 2"),
        }
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any { self }
}

impl AsAny for dyn Params {}


impl EffectType {
    fn variants() -> &'static [&'static str] {
        &["Effect 1", "Effect 2"]
    }

    fn to_index(self) -> usize {
        match self {
            EffectType::Effect1 => 0,
            EffectType::Effect2 => 1,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => EffectType::Effect1,
            1 => EffectType::Effect2,
            _ => EffectType::Effect1,
        }
    }

    fn ids() -> Option<&'static [&'static str]> {
        Some(&["effect1", "effect2"])
    }
}

/// Factory-Funktion, die basierend auf dem Effekt-Typ das entsprechende Effekt-Objekt erzeugt.
pub fn create_effect(effect_type: EffectType) -> Box<dyn Effect> {
    match effect_type {
        EffectType::Effect1 => Box::new(effect1::Effect1::new()),
        EffectType::Effect2 => Box::new(effect2::Effect2::new()),
    }
}
