//! Working on the assumption/belief that the transponder reply light does not
//! light up when the transponder is in standby mode
//!
//! This is default X-Plane behaviour, not a fault with the MU-2
//!
//! When the aircraft loads, the transponder mode is 0/Off. When moving the
//! switch to Standby, the transponder mode is 1/Standby, but the reply light
//! keeps flashing - implying the transponder is still responding to
//! interrogation pulses.

use xplm::data::{borrowed::DataRef, owned::OwnedData, DataRead, DataReadWrite};

use crate::component::PluginComponent;

pub struct TransponderLight {
    mode: DataRef<i32>,
    source: DataRef<f32>,
    output: OwnedData<f32>,
}

impl TransponderLight {
    pub fn new() -> Self {
        Self {
            mode: DataRef::find("sim/cockpit2/radios/actuators/transponder_mode").unwrap(),
            source: DataRef::find("sim/cockpit2/radios/indicators/transponder_brightness").unwrap(),
            output: OwnedData::create("jdeeth/mu2tweaks/transponder_brightness").unwrap(),
        }
    }
}

impl PluginComponent for TransponderLight {
    fn update(&mut self, _tdelta: std::time::Duration) {
        self.output.set(match self.mode.get() {
            1 => 0f32,
            _ => self.source.get(),
        })
    }
}
