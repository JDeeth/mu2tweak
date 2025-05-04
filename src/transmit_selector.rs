use std::time::Duration;

use xplm::data::{borrowed::DataRef, DataRead, DataReadWrite, ReadWrite};

use crate::component::PluginComponent;

/// Link transmit knob to functional COM1/COM2 transmit selection
///
/// Note the `audio_com_selection` dataref will also set which radio is
/// selected for receiving i.e. 6 will transmit and receive on COM1 and
/// mute COM2, 7 will do the opposite. We want to continue to receive from
/// both radios, however.
pub struct TransmitSelector {
    switch: DataRef<f32>,
    actuator: DataRef<i32, ReadWrite>,
}

impl Default for TransmitSelector {
    fn default() -> Self {
        Self {
            switch: DataRef::find("xscenery/mu2b60/manips/transmit_knob").unwrap(),
            actuator: DataRef::find("sim/cockpit2/radios/actuators/audio_com_selection_man")
                .unwrap()
                .writeable()
                .unwrap(),
        }
    }
}

impl PluginComponent for TransmitSelector {
    fn update(&mut self, _tdelta: Duration) {
        // Transmit knob
        // 0: COM1
        // 1: COM2
        // Actuator dataref:
        // 6: COM1
        // 7: COM2
        match self.switch.get() {
            -0.1f32..0.1 => self.actuator.set(6),
            0.9f32..1.1 => self.actuator.set(7),
            _ => (),
        };
    }
}
