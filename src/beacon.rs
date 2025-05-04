//! Rotating beacon animation

use std::time::Duration;

use xplm::data::{borrowed::DataRef, owned::OwnedData, DataRead, DataReadWrite};

use crate::component::PluginComponent;

pub struct Beacon {
    source: DataRef<f32>,
    output: OwnedData<f32>,
    cycle_time: Duration,
    pos: f32,
}

impl Beacon {
    pub fn new() -> Self {
        Self {
            source: DataRef::find("xscenery/mu2b60/lights/beacon_oscillator").unwrap(),
            output: OwnedData::create("jdeeth/mu2tweaks/beacon_oscillator").unwrap(),
            cycle_time: Duration::from_secs(1),
            pos: 0f32,
        }
    }
}

impl PluginComponent for Beacon {
    fn update(&mut self, tdelta: std::time::Duration) {
        match self.source.get() {
            0f32..0.01 => self.output.set(0f32),
            _ => {
                self.pos += tdelta.div_duration_f32(self.cycle_time);
                // per Exterior_Lights.obj, the animation runs between 0.04 and 0.90
                if self.pos > 0.9f32 {
                    // reduce by 0.86 to loop back from 0.90 to 0.04
                    self.pos -= 0.86f32;
                }
                self.output.set(self.pos);
            }
        }
    }
}
