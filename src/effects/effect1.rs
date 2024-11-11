use nih_plug::buffer::ChannelSamples;
use crate::params::PluginParams;

pub fn process(samples: ChannelSamples, params: &PluginParams) {
    let gain = params.effect1_gain.value();
    
    for sample in samples {
        *sample *= gain;
    }
}
