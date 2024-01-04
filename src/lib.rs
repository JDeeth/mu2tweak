extern crate xplm;

use xplm::flight_loop::{FlightLoop, FlightLoopCallback};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{xplane_plugin, debugln};

mod radalt;
use radalt::FilteredRadAlt;
mod gpspower;
use gpspower::GpsPower;
mod condition_command;
use condition_command::ConditionLeverCommands;

struct Components {
    _radalt: FilteredRadAlt,
    _gpspower: GpsPower,
    _cond_lvr_cmds: ConditionLeverCommands,
}

impl Components {
    fn new() -> Self {
        Self {
            _radalt: FilteredRadAlt::new(),
            _gpspower: GpsPower::new(),
            _cond_lvr_cmds: ConditionLeverCommands::new(),
        }
    }
}

impl FlightLoopCallback for Components {
    fn flight_loop(&mut self, _state: &mut xplm::flight_loop::LoopState) {
        self._gpspower.update();
    }
}

struct Mu2Tweaks {
    _update_loop: FlightLoop,
}

impl Plugin for Mu2Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        let mut update_loop = FlightLoop::new(Components::new());
        update_loop.schedule_immediate();
        Ok(Mu2Tweaks{_update_loop: update_loop})
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from("MU-2 Tweaks"),
            signature: String::from("com.jdeeth.mu2tweaks"),
            description: String::from("Tweaked UI datarefs for TOGASim MU-2"),
        }
    }
}

xplane_plugin!(Mu2Tweaks);
