use nih_plug::prelude::*;
use std::sync::Arc;

mod effects;
mod params;

use effects::EffectType;
use params::PluginParams;

struct VariableEffects {
    params: Arc<PluginParams>,
}

impl Default for VariableEffects {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
        }
    }
}

impl Plugin for VariableEffects {
    const NAME: &'static str = "Variable Effects";
    const VENDOR: &'static str = "The Muzikar";
    const URL: &'static str = "https://github.com/osemine/variable-effects";
    const EMAIL: &'static str = "oskar.wiedrich@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let effect_type = self.params.effect_type.value();
            
            match effect_type {
                EffectType::Effect1 => effects::effect1::process(channel_samples, &self.params),
                EffectType::Effect2 => effects::effect2::process(channel_samples, &self.params),
            }
        }
    
        ProcessStatus::Normal
    }
    
}

impl ClapPlugin for VariableEffects {
    const CLAP_ID: &'static str = "com.the-muzikar.variable-effects";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A multi-effect plugin with variable effects");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for VariableEffects {
    const VST3_CLASS_ID: [u8; 16] = *b"VariableEffects.";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_clap!(VariableEffects);
nih_export_vst3!(VariableEffects);
