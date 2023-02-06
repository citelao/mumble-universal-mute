use mumble_sys::{traits::{MumblePlugin, MumblePluginDescriptor}};

// TODO remove
use mumble_sys::types as m;
use windows::{ApplicationModel::Calls::{VoipCallCoordinator, VoipPhoneCallMedia, VoipPhoneCall}, h};

struct MutePlugin {
    api: mumble_sys::MumbleAPI,
    coordinator: VoipCallCoordinator,
    call: Option<VoipPhoneCall>,
}

// https://github.com/Dessix/rust-mumble-rpc/blob/master/src/lib.rs
impl MumblePlugin for MutePlugin {
    fn on_server_synchronized(&mut self, _conn: m::ConnectionT) {
        let api = &mut self.api;
        api.log("Server connected").unwrap();
    }

    fn on_server_disconnected(&mut self, _conn: m::ConnectionT) {
        let api = &mut self.api;
        api.log("Server disconnected").unwrap();
    }

    fn on_channel_entered(
            &mut self,
            conn: m::ConnectionT,
            user: m::UserIdT,
            _previous: Option<m::ChannelIdT>,
            current: Option<m::ChannelIdT>,
        ) {
        let api = &mut self.api;
        if !api.is_connection_synchronized(conn) { return; }

        api.log("HI2 besto besto besto").unwrap();

        let local_user_id = api.get_local_user_id(conn).unwrap();
        if user != local_user_id {
            // We don't care about other user changes.
            return;
        }

        let channel_name = current
            .map(|c| api.get_channel_name(conn, c).unwrap())
            .unwrap_or(String::from("<None>"));
        // let user_name = api
        //     .get_user_name(conn, user)
        //     .unwrap_or(String::from("<Unavailable>"));
        // let user_hash = api
        //     .get_user_hash(conn, user)
        //     .unwrap_or(String::from("<Unavailable>"));
        // let server_hash = api
        //     .get_server_hash(conn)
        //     .unwrap_or(String::from("<Unavailable>"));

        println!("Joined a channel!");
        api.log("HI").unwrap();
        let call = self.coordinator.RequestNewOutgoingCall(
            h!("context_link_todo"),
            &channel_name.into(),
            h!("Mumble"), 
            VoipPhoneCallMedia::Audio)
            .expect("Call should be createable");
        call.NotifyCallActive()
            .expect("Call should be startable");
        self.call = Some(call);
    }

    fn on_channel_exited(
            &mut self,
            conn: m::ConnectionT,
            user: m::UserIdT,
            _channel: Option<m::ChannelIdT>,
        ) {
        let api = &mut self.api;
        if !api.is_connection_synchronized(conn) { return; }

        api.log("Channel exited").unwrap();

        let local_user_id = api.get_local_user_id(conn).unwrap();
        if user != local_user_id {
            // We don't care about other user changes.
            return;
        }

        self.call.as_ref().expect("We should be in a call now")
            .NotifyCallEnded().expect("We should be able to end the call");
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

    fn version() -> m::Version {
        println!("Version requested");
        m::Version { major: 0, minor: 0, patch: 1 }
    }

    fn api_version() -> m::Version {
        // Implement this manually to avoid a linker error.
        println!("APIVersion requested");
        m::Version { major: 1, minor: 0, patch: 0 }
    }

    fn init(id: mumble_sys::types::PluginId, api: mumble_sys::types::MumbleAPI) -> Result<Self, mumble_sys::types::ErrorT>
    where
        Self: Sized {
        println!("It's alive!");

        let mut full_api = mumble_sys::MumbleAPI::new(id, api);
        
        // TODO: remove this debug message.
        full_api.log("Hello there!").unwrap();
        
        let coordinator = VoipCallCoordinator::GetDefault().expect("Could not get WinRT call coordinator");
        // TODO: register MuteStateChanged to handle win-alt-k
        // coordinator.MuteStateChanged()

        Ok(MutePlugin {
            api: full_api,
            coordinator: coordinator,
            call: None,
        })
    }
}

mumble_sys::register_mumble_plugin!(MutePlugin);