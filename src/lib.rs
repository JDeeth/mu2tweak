extern crate xplm;

use xplm::plugin::{Plugin, PluginInfo};
use xplm::xplane_plugin;

mod radalt;
use radalt::FilteredRadAlt;

struct Mu2Tweaks {
    _radalt: FilteredRadAlt,
}

impl Mu2Tweaks {
    fn new() -> Self {
        Self {
            _radalt: FilteredRadAlt::new(),
        }
    }
}

impl Plugin for Mu2Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        Ok(Mu2Tweaks::new())
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
