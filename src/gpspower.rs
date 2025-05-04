use xplm::{
    data::{borrowed::DataRef, owned::OwnedData, DataRead, DataReadWrite, ReadOnly},
    debugln,
};

use crate::component::PluginComponent;

pub struct GpsPower {
    gps_power: DataRef<i32, ReadOnly>,
    gps_fuse: Option<DataRef<f32, ReadOnly>>,
    output: OwnedData<i32, ReadOnly>,
}

impl GpsPower {
    pub fn new() -> Self {
        debugln!("[MU2Tweaks] New GpsPower");
        Self {
            gps_power: DataRef::find("sim/cockpit2/radios/actuators/gps_power").unwrap(),
            gps_fuse: DataRef::find("xscenery/mu2b60/manips/fuse_position/gps15_fuse").ok(),
            output: OwnedData::create("com/jdeeth/mu2tweaks/gps_power").unwrap(),
        }
    }
}

impl PluginComponent for GpsPower {
    fn update(&mut self, _tdelta: std::time::Duration) {
        if self.gps_fuse.is_none() {
            self.gps_fuse = DataRef::find("xscenery/mu2b60/manips/fuse_position/gps15_fuse").ok();
            self.output.set(0);
        } else {
            self.output
                .set(match self.gps_fuse.as_ref().unwrap().get() > 0.1 {
                    true => self.gps_power.get(),
                    false => 0,
                })
        }
    }
}
