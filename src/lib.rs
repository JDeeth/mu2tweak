extern crate xplm;

use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

struct Mu2Tweaks;

impl Plugin for Mu2Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!("Hello, world! From the MU-2 Tweaks plugin.");
        Ok(Mu2Tweaks)
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