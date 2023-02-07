use std::sync::{Arc, Mutex};

use mumble_sys::{traits::{MumblePlugin, MumblePluginDescriptor}};

// TODO remove
use mumble_sys::types as m;
use windows::{ApplicationModel::Calls::{VoipCallCoordinator, VoipPhoneCallMedia, VoipPhoneCall, MuteChangeEventArgs}, h, Foundation::TypedEventHandler};

// struct ScopedTypedEventHandler<TSender: windows::core::RuntimeType + 'static, TResult: windows::core::RuntimeType + 'static> {
//     handler: TypedEventHandler<TSender, TResult>
// }

struct GlobalState {
    api: mumble_sys::MumbleAPI,
    coordinator: VoipCallCoordinator,
}

struct MutePlugin {
    state: Arc<Mutex<GlobalState>>,
    coordinator: VoipCallCoordinator,
    call: Option<VoipPhoneCall>,
}

// https://github.com/Dessix/rust-mumble-rpc/blob/master/src/lib.rs
impl MumblePlugin for MutePlugin {
    fn on_server_synchronized(&mut self, _conn: m::ConnectionT) {
        let api = &mut self.state.lock().unwrap().api;
        api.log("Server connected").unwrap();

        let call = self.coordinator.RequestNewOutgoingCall(
            h!("context_link_todo"),
            h!("TODO Channel"),
            h!("Mumble"), 
            VoipPhoneCallMedia::Audio)
            .expect("Call should be createable");

        let is_muted = api.get_local_user_muted().unwrap();
        if is_muted {
            self.coordinator.NotifyMuted().unwrap();
        } else {
            self.coordinator.NotifyUnmuted().unwrap();
        }

        call.NotifyCallActive()
            .expect("Call should be startable");
        self.call = Some(call);
    }

    fn on_server_disconnected(&mut self, _conn: m::ConnectionT) {
        let api = &mut self.state.lock().unwrap().api;
        api.log("Server disconnected").unwrap();

        if let Some(call) = self.call.as_ref() {
            call.NotifyCallEnded().expect("We should be able to end the call");
            self.call = None;
        }
    }

    // fn on_channel_renamed(&mut self, conn: m::ConnectionT, channel: m::ChannelIdT) {
    //     let api = &mut self.state.lock().unwrap().api;
    //     api.log("Channel renamed").unwrap();
    // }

    fn on_channel_entered(
            &mut self,
            conn: m::ConnectionT,
            user: m::UserIdT,
            _previous: Option<m::ChannelIdT>,
            current: Option<m::ChannelIdT>,
        ) {
        let api = &mut self.state.lock().unwrap().api;
        if !api.is_connection_synchronized(conn) { return; }

        api.log("HI2 besto besto besto").unwrap();

        let local_user_id = api.get_local_user_id(conn).unwrap();
        if user != local_user_id {
            // We don't care about other user changes.
            return;
        }

        let _channel_name = current
            .map(|c| api.get_channel_name(conn, c).unwrap())
            .unwrap_or(String::from("<None>"));
    
        // if let Some(call) = self.call {
        //     // call.SetContactName("value");
        // }

        // let user_name = api
        //     .get_user_name(conn, user)
        //     .unwrap_or(String::from("<Unavailable>"));
        // let user_hash = api
        //     .get_user_hash(conn, user)
        //     .unwrap_or(String::from("<Unavailable>"));
        // let server_hash = api
        //     .get_server_hash(conn)
        //     .unwrap_or(String::from("<Unavailable>"));

        // println!("Joined a channel!");
        // api.log("HI").unwrap();
        // let call = self.coordinator.RequestNewOutgoingCall(
        //     h!("context_link_todo"),
        //     &channel_name.into(),
        //     h!("Mumble"), 
        //     VoipPhoneCallMedia::Audio)
        //     .expect("Call should be createable");
        // call.NotifyCallActive()
        //     .expect("Call should be startable");
        // self.call = Some(call);
    }

    fn on_channel_exited(
            &mut self,
            conn: m::ConnectionT,
            user: m::UserIdT,
            _channel: Option<m::ChannelIdT>,
        ) {
        let api = &mut self.state.lock().unwrap().api;
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

        let full_api = mumble_sys::MumbleAPI::new(id, api);
        let coordinator = VoipCallCoordinator::GetDefault().expect("Could not get WinRT call coordinator");
        let state = Arc::new(Mutex::new(GlobalState {
            api: full_api,
            coordinator: coordinator.clone(),
        }));

        // let mut full_api_ref = full_api.lock().unwrap();
        
        // TODO: remove this debug message.
        // full_api_ref.log("Hello there!").unwrap();

        let state_copy = state.clone();
        coordinator.MuteStateChanged(&TypedEventHandler::new(move |_, args: &Option<MuteChangeEventArgs>| {
            if let Some(a) = args {
                let mut api_ref = &mut state_copy.lock().unwrap().api;
                api_ref.log("Mute request").unwrap();
                api_ref.request_local_user_mute(a.Muted().unwrap()).unwrap();
            }
            Ok(())
        })).unwrap();

        Ok(MutePlugin {
            state: state,
            coordinator: coordinator,
            call: None,
        })
    }
}

mumble_sys::register_mumble_plugin!(MutePlugin);