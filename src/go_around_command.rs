use xplm::{
    command::{CommandHandler, OwnedCommand},
    data::{
        borrowed::{DataRef, FindError},
        DataReadWrite, ReadWrite,
    },
};

use crate::component::PluginComponent;

struct ButtonCmd {
    dataref_name: String,
    dataref: Option<DataRef<f32, ReadWrite>>,
}

impl ButtonCmd {
    pub fn new(dataref_name: String) -> Self {
        Self {
            dataref_name,
            dataref: None,
        }
    }
    fn find(&mut self) -> Result<DataRef<f32, ReadWrite>, FindError> {
        DataRef::find(&self.dataref_name)?.writeable()
    }
}

impl CommandHandler for ButtonCmd {
    fn command_begin(&mut self) {
        if self.dataref.is_none() {
            self.dataref = self.find().ok();
        }
        if let Some(dataref) = self.dataref.as_mut() {
            dataref.set(1f32);
        }
    }

    fn command_continue(&mut self) {}

    fn command_end(&mut self) {
        if let Some(dataref) = self.dataref.as_mut() {
            dataref.set(0f32);
        }
    }
}

pub struct GoAroundButton {
    _go_around: OwnedCommand,
}

impl Default for GoAroundButton {
    fn default() -> Self {
        Self {
            _go_around: OwnedCommand::new(
                "jdeeth/mu2tweaks/ga_button",
                "Throttle GA (Go Around) button",
                ButtonCmd::new("xscenery/mu2b60/manips/ga_button".to_string()),
            )
            .unwrap(),
        }
    }
}

impl PluginComponent for GoAroundButton {
    fn update(&mut self, tdelta: std::time::Duration) {
        let _ = tdelta;
    }
}
