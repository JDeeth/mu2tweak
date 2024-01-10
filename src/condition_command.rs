use xplm::{
    command::{CommandHandler, OwnedCommand},
    data::{
        borrowed::{DataRef, FindError},
        DataReadWrite, ReadWrite,
    },
};

struct SetDatarefCmd {
    dataref_name: String,
    dataref: Option<DataRef<f32, ReadWrite>>,
    position: f32,
}

impl CommandHandler for SetDatarefCmd {
    fn command_begin(&mut self) {}
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {
        if self.dataref.is_none() {
            self.dataref = self.find().ok();
        }
        if let Some(dataref) = self.dataref.as_mut() {
            dataref.set(self.position);
        }
    }
}

impl SetDatarefCmd {
    fn find(&mut self) -> Result<DataRef<f32, ReadWrite>, FindError> {
        DataRef::find(&self.dataref_name)?.writeable()
    }

    fn make(
        dataref_name: &str,
        position: f32,
        command_name: &str,
        description: &str,
    ) -> OwnedCommand {
        let handler = SetDatarefCmd {
            dataref_name: dataref_name.to_string(),
            dataref: None,
            position,
        };
        OwnedCommand::new(command_name, description, handler).unwrap()
    }
}

enum Side {
    Left,
    Right,
}

impl Side {
    fn letter_word(&self) -> (char, &str) {
        match self {
            Side::Left => ('L', "left"),
            Side::Right => ('R', "right"),
        }
    }
}

struct ConditionLever {
    _stop: OwnedCommand,
    _taxi: OwnedCommand,
    _cruise: OwnedCommand,
    _full: OwnedCommand,
}

impl ConditionLever {
    fn new(side: Side) -> Self {
        fn build_cmd(dataref_name: &str, side: &Side, position: f32, label: &str) -> OwnedCommand {
            SetDatarefCmd::make(
                dataref_name,
                position,
                &format!(
                    "jdeeth/mu2tweaks/{}_condition_lever_to_{}",
                    side.letter_word().1,
                    label
                ),
                &format!("Move {} condition lever to {}", side.letter_word().1, label),
            )
        }
        let dataref_name = format!(
            "xscenery/mu2b60/manips/{}_condition_lever_rotate",
            side.letter_word().0
        );

        Self {
            _stop: build_cmd(&dataref_name, &side, 0.0, "EMERGSTOP"),
            _taxi: build_cmd(&dataref_name, &side, 0.38, "TAXI"),
            _cruise: build_cmd(&dataref_name, &side, 0.8, "MINCRUISE"),
            _full: build_cmd(&dataref_name, &side, 1.0, "TAKEOFFLAND"),
        }
    }
}

pub struct ConditionLeverCommands {
    _left_lever: ConditionLever,
    _right_lever: ConditionLever,
}

impl ConditionLeverCommands {
    pub fn new() -> Self {
        Self {
            _left_lever: ConditionLever::new(Side::Left),
            _right_lever: ConditionLever::new(Side::Right),
        }
    }
}
