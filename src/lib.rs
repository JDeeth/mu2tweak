extern crate xplm;

use std::time::Duration;

use beacon::Beacon;
use component::PluginComponent;
use condition_command::ConditionLeverCommands;
use go_around_command::GoAroundButton;
use gpspower::GpsPower;
use radalt::FilteredRadAlt;
use radio_anim::RadioAnim;
use radio_command::RadioCommands;
use transmit_selector::TransmitSelector;
use transponder_light::TransponderLight;
use xplm::data::borrowed::DataRef;
use xplm::data::DataRead;
use xplm::flight_loop::{FlightLoop, FlightLoopCallback};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

mod beacon;
mod component;
mod condition_command;
mod go_around_command;
mod gpspower;
mod radalt;
mod radio_anim;
mod radio_command;
mod transmit_selector;
mod transponder_light;

struct Components {
    sim_speed: DataRef<f32>,
    components: [Box<dyn PluginComponent>; 9],
}

impl Components {
    fn new() -> Self {
        Self {
            sim_speed: DataRef::find("sim/time/sim_speed_actual").unwrap(),
            components: [
                Box::new(ConditionLeverCommands::new()),
                Box::new(FilteredRadAlt::new()),
                Box::new(GoAroundButton::default()),
                Box::new(GpsPower::new()),
                Box::new(RadioAnim::new()),
                Box::new(RadioCommands::new()),
                Box::new(TransmitSelector::default()),
                Box::new(Beacon::new()),
                Box::new(TransponderLight::new()),
            ],
        }
    }
}

impl FlightLoopCallback for Components {
    fn flight_loop(&mut self, state: &mut xplm::flight_loop::LoopState) {
        let tdelta = match self.sim_speed.get() {
            x if x.is_nan() => Duration::default(),
            x => state.since_last_call().mul_f32(x),
        };
        for c in &mut self.components {
            c.update(tdelta);
        }
    }
}

struct Mu2Tweaks {
    _update_loop: FlightLoop,
}

impl Plugin for Mu2Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!("[MU2Tweaks] Plugin start starting...");
        let mut update_loop = FlightLoop::new(Components::new());
        debugln!("[MU2Tweaks] Update loop created");
        update_loop.schedule_immediate();
        debugln!("[MU2Tweaks] Update loop scheduled");
        Ok(Mu2Tweaks {
            _update_loop: update_loop,
        })
    }

    fn info(&self) -> PluginInfo {
        let ts = env!("VERGEN_BUILD_TIMESTAMP")
            .get(0..16)
            .unwrap_or("1970-01-01T00:00");
        let debug = match env!("VERGEN_CARGO_DEBUG") {
            "true" => "debug",
            _ => "release",
        };
        let opt_level = env!("VERGEN_CARGO_OPT_LEVEL");

        PluginInfo {
            name: String::from("MU-2 Tweaks"),
            signature: String::from("com.jdeeth.mu2tweaks"),
            description: format!("Tweaked UI datarefs for TOGASim MU-2, compiled {ts}Z, {debug} opt level {opt_level}"),
        }
    }
}

xplane_plugin!(Mu2Tweaks);
