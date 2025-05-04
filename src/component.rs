use std::time::Duration;

pub trait PluginComponent {
    fn update(&mut self, tdelta: Duration) {
        let _ = tdelta;
    }
}
