use nih_plug::prelude::Enum;

pub mod effect1;
pub mod effect2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Effect1,
    Effect2,
}

impl Enum for EffectType {
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
            _ => EffectType::Effect1, // Default case
        }
    }

    fn ids() -> Option<&'static [&'static str]> {
        Some(&["effect1", "effect2"])
    }
}

impl std::fmt::Display for EffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectType::Effect1 => write!(f, "Effect 1"),
            EffectType::Effect2 => write!(f, "Effect 2"),
        }
    }
}
