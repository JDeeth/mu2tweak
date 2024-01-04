use xplm::{
    command::{CommandHandler, OwnedCommand},
    data::{
        borrowed::{DataRef, FindError},
        DataReadWrite, ReadWrite,
    },
};

trait IDataRef {
    fn find(name: &str) -> Self;
}

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

    fn new(
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

struct ConditionLever {
    _cruise: OwnedCommand,
    _full: OwnedCommand,
}

impl ConditionLever {
    fn new(side: Side) -> Self {
        let (lr, left_right) = match side {
            Side::Left => ('L', "left"),
            Side::Right => ('R', "right"),
        };
        let rotation = format!("xscenery/mu2b60/manips/{lr}_condition_lever_rotate");

        Self {
            _cruise: SetDatarefCmd::new(
                &rotation,
                0.8,
                &format!("jdeeth/mu2tweaks/{left_right}_condition_lever_to_MINCRUISE"),
                &format!("Move {left_right} condition lever to MIN CRUISE"),
            ),
            _full: SetDatarefCmd::new(
                &rotation,
                1.0,
                &format!("jdeeth/mu2tweaks/{left_right}_condition_lever_to_TAKEOFFLAND"),
                &format!("Move {left_right} condition lever to TAKE OFF LAND"),
            ),
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
