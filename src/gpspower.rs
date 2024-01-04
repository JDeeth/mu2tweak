use xplm::data::{borrowed::DataRef, owned::OwnedData, DataRead, DataReadWrite, ReadOnly};

pub struct GpsPower {
    gps_power: DataRef<i32, ReadOnly>,
    gps_fuse: DataRef<f32, ReadOnly>,
    output: OwnedData<i32, ReadOnly>,
}

impl GpsPower {
    pub fn new() -> Self {
        Self {
            gps_power: DataRef::find("sim/cockpit2/radios/actuators/gps_power").unwrap(),
            gps_fuse: DataRef::find("xscenery/mu2b60/manips/fuse_position/gps15_fuse").unwrap(),
            output: OwnedData::create("com/jdeeth/mu2tweaks/gps_power").unwrap(),
        }
    }

    pub fn update(&mut self) {
        self.output.set(match self.gps_fuse.get() > 0.1 {
            true => self.gps_power.get(),
            false => 0,
        })
    }
}
