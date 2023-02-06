use mumble_sys::{traits::MumblePlugin, types::Mumble_ErrorCode};

// fn main() {
//     println!("Hello, world!");
// }

struct mute_plugin {}

impl MumblePlugin for mute_plugin {
    fn init(&mut self) -> Mumble_ErrorCode {
        println!("Init");
        return Mumble_ErrorCode::EC_OK;
    }
    fn shutdown(&mut self) {
        println!("Shutdown");
    }

    fn set_api(&mut self, api: crate::MumbleAPI) {
        // TODO
    }
}
