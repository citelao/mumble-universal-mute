use mumble_sys::{traits::{MumblePlugin, MumblePluginDescriptor}};

// TODO remove
use mumble_sys::types as m;

struct MutePlugin {
    api: mumble_sys::MumbleAPI,
}

// https://github.com/Dessix/rust-mumble-rpc/blob/master/src/lib.rs
impl MumblePlugin for MutePlugin {
    fn on_channel_entered(
            &mut self,
            conn: m::ConnectionT,
            _user: m::UserIdT,
            _previous: Option<m::ChannelIdT>,
            _current: Option<m::ChannelIdT>,
        ) {
        let api = &mut self.api;
        if !api.is_connection_synchronized(conn) { return; }
        println!("Joined a channel!")
    }

    fn shutdown(&self) {
        println!("Shutdown");
    }
}

impl MumblePluginDescriptor for MutePlugin {
    fn name() -> &'static str {
        "Universal Mute for Mumble"
    }

    fn author() -> &'static str {
        "Ben Stolovitz (citelao)"
    }

    fn description() -> &'static str {
        "Enable universal mute for Mumble"
    }

    fn init(id: mumble_sys::types::PluginId, api: mumble_sys::types::MumbleAPI) -> Result<Self, mumble_sys::types::ErrorT>
    where
        Self: Sized {
        println!("It's alive!");
        Ok(MutePlugin { api: mumble_sys::MumbleAPI::new(id, api) })
    }
}

mumble_sys::register_mumble_plugin!(MutePlugin);