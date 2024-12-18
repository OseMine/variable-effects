use nih_plug::prelude::*;
use std::sync::Arc;

mod effects;
mod params;


use effects::Effect;
use params::PluginParams;

struct VariableEffects {
    params: Arc<PluginParams>,
    effects: Vec<Box<dyn Effect>>,
}

impl Default for VariableEffects {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            effects: vec![
                Box::new(effects::gain::Gain::new()),
                Box::new(effects::chorus::Chorus::new()),
            ],
        }
    }
}

impl Plugin for VariableEffects {
    const NAME: &'static str = "Variable Effects";
    const VENDOR: &'static str = "The Muzikar";
    const URL: &'static str = "https://github.com/OseMine/variable-effects";
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
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Aktuellen Effekt auswählen
        let effect_index = self.params.effect_type.value() as usize;
        let sample_rate = context.transport().sample_rate;

        for mut channel_samples in buffer.iter_samples() {
            let mut samples: Vec<f32> = channel_samples.iter_mut().map(|s| *s).collect();

            // Effekt verarbeiten, wenn Index gültig ist
            if effect_index < self.effects.len() {
                self.effects[effect_index].process(
                    &mut samples,
                    sample_rate,
                    self.params.get_params(effect_index),
                );
            }

            for (out, &processed) in channel_samples.iter_mut().zip(samples.iter()) {
                *out = processed;
            }
        }

        ProcessStatus::Normal
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        None
    }
}

impl ClapPlugin for VariableEffects {
    const CLAP_ID: &'static str = "com.the-muzikar.variable-effects";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A plugin with modular effects");
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
