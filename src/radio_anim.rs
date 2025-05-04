use xplm::{
    data::{borrowed::DataRef, owned::OwnedData, DataRead, DataReadWrite, ReadOnly},
    debugln,
};

use crate::component::PluginComponent;

/// Replacement datarefs for the NAV/COM roller digits in the MU-2 OEM radio stack
struct MhzDrums {
    source: DataRef<i32, ReadOnly>,
    drum_100: OwnedData<f32, ReadOnly>,
    drum_010: OwnedData<f32, ReadOnly>,
    drum_001: OwnedData<f32, ReadOnly>,
}
impl MhzDrums {
    fn new(source_name: &str, output_prefix: String) -> Self {
        Self {
            source: DataRef::find(source_name).expect("dataref to exist"),
            drum_100: OwnedData::create(format!("{output_prefix}_100").as_str())
                .expect("dataref to create"),
            drum_010: OwnedData::create(format!("{output_prefix}_010").as_str())
                .expect("dataref to create"),
            drum_001: OwnedData::create(format!("{output_prefix}_001").as_str())
                .expect("dataref to create"),
        }
    }

    fn update(&mut self) {
        let input = self.source.get();
        let nth = |val: i32, digit: u32| ((val / (10i32.pow(digit))) % 10) as f32;
        self.drum_001.set(nth(input, 0));
        self.drum_010.set(nth(input, 1));
        self.drum_100.set(nth(input, 2));
    }
}

struct ComKhzDrums {
    source: DataRef<i32, ReadOnly>,
    drum_10: OwnedData<f32, ReadOnly>,
    drum_01: OwnedData<f32, ReadOnly>,
}
impl ComKhzDrums {
    fn new(source_name: &str, output_prefix: String) -> Self {
        Self {
            source: DataRef::find(source_name).expect("dataref to exist"),
            drum_10: OwnedData::create(format!("{output_prefix}_10").as_str())
                .expect("dataref to create"),
            drum_01: OwnedData::create(format!("{output_prefix}_01").as_str())
                .expect("dataref to create"),
        }
    }
    fn update(&mut self) {
        let freq = self.source.get();
        let digit_01 = (freq % 100) as f32 / 10.;
        let digit_10 = (freq / 100) as f32 + 0f32.max(0.3 * (digit_01 - 9.));
        self.drum_01.set(digit_01);
        self.drum_10.set(digit_10);
    }
}

struct NavKhzDrums {
    source: DataRef<i32, ReadOnly>,
    drum_10: OwnedData<f32, ReadOnly>,
    drum_01: OwnedData<f32, ReadOnly>,
}
impl NavKhzDrums {
    fn new(source_name: &str, output_prefix: String) -> Self {
        Self {
            source: DataRef::find(source_name).expect("dataref to exist"),
            drum_10: OwnedData::create(format!("{output_prefix}_10").as_str())
                .expect("dataref to create"),
            drum_01: OwnedData::create(format!("{output_prefix}_01").as_str())
                .expect("dataref to create"),
        }
    }
    fn update(&mut self) {
        let freq = self.source.get();
        self.drum_01.set((freq % 10) as f32);
        self.drum_10.set((freq / 10) as f32);
    }
}

pub struct RadioAnim {
    mhz: [MhzDrums; 4],
    com_khz: [ComKhzDrums; 2],
    nav_khz: [NavKhzDrums; 2],
}

impl RadioAnim {
    pub fn new() -> Self {
        debugln!("[MU2Tweaks] New RadioAnim");
        let prefix = |drum: &str| format!("com/jdeeth/mu2tweaks/{drum}");
        Self {
            mhz: [
                MhzDrums::new(
                    "sim/cockpit2/radios/actuators/com1_frequency_Mhz",
                    prefix("com1_mhz"),
                ),
                MhzDrums::new(
                    "sim/cockpit2/radios/actuators/com2_frequency_Mhz",
                    prefix("com2_mhz"),
                ),
                MhzDrums::new(
                    "sim/cockpit2/radios/actuators/nav1_frequency_Mhz",
                    prefix("nav1_mhz"),
                ),
                MhzDrums::new(
                    "sim/cockpit2/radios/actuators/nav2_frequency_Mhz",
                    prefix("nav2_mhz"),
                ),
            ],
            com_khz: [
                ComKhzDrums::new(
                    "sim/cockpit2/radios/actuators/com1_frequency_khz",
                    prefix("com1_khz"),
                ),
                ComKhzDrums::new(
                    "sim/cockpit2/radios/actuators/com2_frequency_khz",
                    prefix("com2_khz"),
                ),
            ],
            nav_khz: [
                NavKhzDrums::new(
                    "sim/cockpit2/radios/actuators/nav1_frequency_khz",
                    prefix("nav1_khz"),
                ),
                NavKhzDrums::new(
                    "sim/cockpit2/radios/actuators/nav2_frequency_khz",
                    prefix("nav2_khz"),
                ),
            ],
        }
    }
}
impl PluginComponent for RadioAnim {
    fn update(&mut self, _tdelta: std::time::Duration) {
        self.mhz.iter_mut().for_each(|d| d.update());
        self.com_khz.iter_mut().for_each(|d| d.update());
        self.nav_khz.iter_mut().for_each(|d| d.update());
    }
}
