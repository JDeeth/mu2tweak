use xplm::command::{Command, CommandHandler, OwnedCommand};

/// A blooming awful hack where we toggle active/standby, move the standby freq
/// with the stock (normal-behaving) X-Plane command, then toggle back
struct ChangeViaStby {
    flip_stby: Command,
    move_stby: Command,
}

impl ChangeViaStby {
    fn make(radio: &'static str, direction: &'static str) -> OwnedCommand {
        let with_833 = match radio.starts_with("com") {
            true => "_833",
            false => "",
        };
        let result = Self {
            // yes it's "standy"
            flip_stby: Command::find(format!("sim/radios/{radio}_standy_flip").as_str())
                .expect("command to exist"),
            move_stby: Command::find(
                format!("sim/radios/stby_{radio}_fine_{direction}{with_833}").as_str(),
            )
            .expect("command to exist"),
        };
        OwnedCommand::new(
            format!("sim/radios/actv_{radio}_fine_{direction}{with_833}").as_str(),
            "",
            result,
        )
        .expect("command override to work")
    }
}

impl CommandHandler for ChangeViaStby {
    fn command_begin(&mut self) {}
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {
        self.flip_stby.trigger();
        self.move_stby.trigger();
        self.flip_stby.trigger();
    }
}

pub struct RadioCommands {
    _nav1_up: OwnedCommand,
    _nav1_dn: OwnedCommand,
    _nav2_up: OwnedCommand,
    _nav2_dn: OwnedCommand,
    _com1_up: OwnedCommand,
    _com1_dn: OwnedCommand,
    _com2_up: OwnedCommand,
    _com2_dn: OwnedCommand,
}

impl RadioCommands {
    pub fn new() -> Self {
        Self {
            _nav1_up: ChangeViaStby::make("nav1", "up"),
            _nav1_dn: ChangeViaStby::make("nav1", "down"),
            _nav2_up: ChangeViaStby::make("nav2", "up"),
            _nav2_dn: ChangeViaStby::make("nav2", "down"),
            _com1_up: ChangeViaStby::make("com1", "up"),
            _com1_dn: ChangeViaStby::make("com1", "down"),
            _com2_up: ChangeViaStby::make("com2", "up"),
            _com2_dn: ChangeViaStby::make("com2", "down"),
        }
    }
}
