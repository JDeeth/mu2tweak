use std::time::Duration;

use xplm::data::borrowed::DataRef;
use xplm::data::owned::OwnedData;
use xplm::data::{DataRead, DataReadWrite, ReadOnly};
use xplm::flight_loop::{FlightLoop, FlightLoopCallback, LoopState};

struct FilteredRadAltData {
    source: DataRef<f32, ReadOnly>,
    output: OwnedData<f32, ReadOnly>,
    // separate datarefs for first two and last two digits
    // so we can display "  00"
    output_12: OwnedData<f32, ReadOnly>,
    output_34: OwnedData<f32, ReadOnly>,
}

impl FilteredRadAltData {
    fn new() -> Self {
        let source_dr = "sim/cockpit2/gauges/indicators/radio_altimeter_height_ft_pilot";
        let output_dr = "com/jdeeth/mu2tweaks/radio_altimeter_height_ft_pilot";
        Self {
            source: DataRef::find(source_dr).unwrap(),
            output: OwnedData::create(output_dr).unwrap(),
            output_12: OwnedData::create(&format!("{output_dr}_12")).unwrap(),
            output_34: OwnedData::create(&format!("{output_dr}_34")).unwrap(),
        }
    }
}
impl FlightLoopCallback for FilteredRadAltData {
    fn flight_loop(&mut self, _state: &mut LoopState) {
        let precision = match self.source.get() {
            x if x < 100f32 => 10,
            x if x < 200f32 => 20,
            _ => 50,
        };
        let height_offset = 6;
        // round offset raw value to nearest multiple of $precision
        let displayed_height = precision * ((self.source.get() as i32 + height_offset) / precision);
        if displayed_height > 2500 {
            // blank out gen_LED instruments by asking them to display 0
            self.output.set(0f32);
            self.output_12.set(0f32);
            self.output_34.set(0f32);
            return;
        }
        self.output.set(displayed_height as f32);
        self.output_12.set((displayed_height / 100) as f32);
        self.output_34.set(match displayed_height {
            0 => 100, // make last 2 digits show as 0
            _ => displayed_height,
        } as f32);
    }
}

pub struct FilteredRadAlt {
    _flight_loop: FlightLoop,
}

impl FilteredRadAlt {
    pub fn new() -> Self {
        let data = FilteredRadAltData::new();
        let mut flight_loop = FlightLoop::new(data);
        flight_loop.schedule_after(Duration::from_micros(500));
        Self {
            _flight_loop: flight_loop,
        }
    }
}
