extern crate xplm;

use component::PluginComponent;
use condition_command::ConditionLeverCommands;
use gpspower::GpsPower;
use radalt::FilteredRadAlt;
use radio_anim::RadioAnim;
use radio_command::RadioCommands;
use transmit_selector::TransmitSelector;
use xplm::flight_loop::{FlightLoop, FlightLoopCallback};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

mod component;
mod condition_command;
mod gpspower;
mod radalt;
mod radio_anim;
mod radio_command;
mod transmit_selector;

struct Components {
    components: [Box<dyn PluginComponent>; 6],
}

impl Components {
    fn new() -> Self {
        Self {
            components: [
                Box::new(ConditionLeverCommands::new()),
                Box::new(FilteredRadAlt::new()),
                Box::new(GpsPower::new()),
                Box::new(RadioAnim::new()),
                Box::new(RadioCommands::new()),
                Box::new(TransmitSelector::default()),
            ],
        }
    }
}

impl FlightLoopCallback for Components {
    fn flight_loop(&mut self, state: &mut xplm::flight_loop::LoopState) {
        let tdelta = state.since_last_call();
        let _ = self.components.iter_mut().map(|c| c.update(tdelta));
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
