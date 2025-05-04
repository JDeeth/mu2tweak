use xplm::{
    command::{CommandHandler, OwnedCommand},
    data::{borrowed::DataRef, DataRead, DataReadWrite, ReadWrite},
    debugln,
};

use crate::component::PluginComponent;

fn change_com_khz(freq: i32, step: i32) -> i32 {
    let (f100, f011) = (freq.div_euclid(100), freq.rem_euclid(100));
    let valid_011 = [
        0i32, 5, 10, 15, 25, 30, 35, 40, 50, 55, 60, 65, 75, 80, 85, 90,
    ];
    let valid_count = valid_011.len() as i32;
    let new_channel =
        f100 * valid_count + valid_011.iter().position(|&x| f011 <= x).unwrap_or(0) as i32 + step;
    let new_freq = 100 * new_channel.div_euclid(valid_count)
        + valid_011[new_channel.rem_euclid(valid_count) as usize];
    new_freq.rem_euclid(1000)
}

fn change_nav_khz(freq: i32, step: i32) -> i32 {
    let (f10, f01) = (freq.div_euclid(10), freq.rem_euclid(10));
    let valid_01 = [0i32, 5];
    let valid_count = valid_01.len() as i32;
    let new_channel =
        f10 * valid_count + valid_01.iter().position(|&x| f01 <= x).unwrap_or(0) as i32 + step;
    let new_freq = 10 * new_channel.div_euclid(valid_count)
        + valid_01[new_channel.rem_euclid(valid_count) as usize];
    new_freq.rem_euclid(100)
}

struct KhzTune {
    freq_khz: DataRef<i32, ReadWrite>,
    inc: bool,
    change: fn(i32, i32) -> i32,
}

impl KhzTune {
    fn make(
        radio: &'static str,
        direction: &'static str,
        change: fn(i32, i32) -> i32,
    ) -> OwnedCommand {
        let result = Self {
            freq_khz: DataRef::find(
                format!("sim/cockpit2/radios/actuators/{radio}_frequency_khz").as_str(),
            )
            .expect("dataref to always exist")
            .writeable()
            .expect("dataref to be writeable"),
            inc: direction == "up",
            change,
        };
        OwnedCommand::new(
            &format!("mu2tweaks/{radio}_fine_{direction}"),
            &format!("mu2tweaks/{radio}_fine_{direction}"),
            result,
        )
        .expect("command to be created")
    }
}

impl CommandHandler for KhzTune {
    fn command_begin(&mut self) {}
    fn command_continue(&mut self) {}
    fn command_end(&mut self) {
        let step = match self.inc {
            true => 1,
            false => -1,
        };
        self.freq_khz.set((self.change)(self.freq_khz.get(), step));
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
        debugln!("[MU2Tweaks] New RadioCommands");
        Self {
            _nav1_up: KhzTune::make("nav1", "up", change_nav_khz),
            _nav1_dn: KhzTune::make("nav1", "down", change_nav_khz),
            _nav2_up: KhzTune::make("nav2", "up", change_nav_khz),
            _nav2_dn: KhzTune::make("nav2", "down", change_nav_khz),
            _com1_up: KhzTune::make("com1", "up", change_com_khz),
            _com1_dn: KhzTune::make("com1", "down", change_com_khz),
            _com2_up: KhzTune::make("com2", "up", change_com_khz),
            _com2_dn: KhzTune::make("com2", "down", change_com_khz),
        }
    }
}
impl PluginComponent for RadioCommands {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scratchpad() {
        let freq: i32 = 123;
        let (a, b) = (freq.div_euclid(100), freq.rem_euclid(100));
        assert_eq!(a, 1);
        assert_eq!(b, 23)
    }
    #[test]
    fn com_inc() {
        assert_eq!(change_com_khz(0, 1), 5);
        assert_eq!(change_com_khz(1, 1), 10);
        assert_eq!(change_com_khz(5, 1), 10);
        assert_eq!(change_com_khz(90, 1), 100);
        assert_eq!(change_com_khz(990, 1), 0);
    }
    #[test]
    fn com_dec() {
        assert_eq!(change_com_khz(0, -1), 990);
        assert_eq!(change_com_khz(1, -1), 0);
        assert_eq!(change_com_khz(5, -1), 0);
        assert_eq!(change_com_khz(100, -1), 90);
    }
    #[test]
    fn nav_inc() {
        assert_eq!(change_nav_khz(0, 1), 2);
        assert_eq!(change_nav_khz(1, 1), 5);
        assert_eq!(change_nav_khz(2, 1), 5);
        assert_eq!(change_nav_khz(5, 1), 7);
        assert_eq!(change_nav_khz(97, 1), 0);
    }
    #[test]
    fn nav_dec() {
        assert_eq!(change_nav_khz(0, -1), 97);
        assert_eq!(change_nav_khz(1, -1), 0);
        assert_eq!(change_nav_khz(2, -1), 0);
        assert_eq!(change_nav_khz(5, -1), 2);
    }
}
