extern crate xplm;

use xplm::flight_loop::{FlightLoop, FlightLoopCallback};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

mod radalt;
use radalt::FilteredRadAlt;
mod gpspower;
use gpspower::GpsPower;
mod condition_command;
use condition_command::ConditionLeverCommands;
mod radio_command;
use radio_command::RadioCommands;
mod radio_anim;
use radio_anim::RadioAnim;
mod transmit_selector;
use transmit_selector::TransmitSelector;

struct Components {
    _radalt: FilteredRadAlt,
    _gpspower: GpsPower,
    _cond_lvr_cmds: ConditionLeverCommands,
    _radio_cmds: RadioCommands,
    _radio_anim: RadioAnim,
    _transmit_selector: TransmitSelector,
}

impl Components {
    fn new() -> Self {
        Self {
            _radalt: FilteredRadAlt::new(),
            _gpspower: GpsPower::new(),
            _cond_lvr_cmds: ConditionLeverCommands::new(),
            _radio_cmds: RadioCommands::new(),
            _radio_anim: RadioAnim::new(),
            _transmit_selector: TransmitSelector::default(),
        }
    }
}

impl FlightLoopCallback for Components {
    fn flight_loop(&mut self, _state: &mut xplm::flight_loop::LoopState) {
        self._gpspower.update();
        self._radio_anim.update();
        self._transmit_selector.update();
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
