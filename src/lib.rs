use std::sync::{Arc, Mutex};

use mumble_sys::{traits::{MumblePlugin, MumblePluginDescriptor}, types::ErrorT};
use windows::{ApplicationModel::Calls::{VoipCallCoordinator, VoipPhoneCallMedia, VoipPhoneCall, MuteChangeEventArgs}, h, Foundation::TypedEventHandler};

// TODO remove; there's a bug in the mumble_sys macro impl.
use mumble_sys::types as m;

struct GlobalState {
    api: mumble_sys::MumbleAPI,
    coordinator: VoipCallCoordinator,
    is_in_call: bool,
}

struct MutePlugin {
    state: Arc<Mutex<GlobalState>>,
    coordinator: VoipCallCoordinator,
    call: Option<VoipPhoneCall>,
}

fn debug_log(api: &mut mumble_sys::MumbleAPI, message: &str) -> Result<(), ErrorT> {
    const DEBUG: bool = false;
    if DEBUG {
        api.log(message)?;
    }
    Ok(())
}

// https://github.com/Dessix/rust-mumble-rpc/blob/master/src/lib.rs
impl MumblePlugin for MutePlugin {
    fn on_server_synchronized(&mut self, _conn: m::ConnectionT) {
        let locked_state = &mut self.state.lock().unwrap();
        // let api = &mut self.state.lock().unwrap().api;
        debug_log(&mut locked_state.api, "Server connected").unwrap();

        let call = self.coordinator.RequestNewOutgoingCall(
            h!("context_link_todo"),
            h!("TODO Channel"),
            h!("Mumble"), 
            VoipPhoneCallMedia::Audio)
            .expect("Call should be createable");

        let is_muted = locked_state.api.get_local_user_muted().unwrap();
        if is_muted {
            self.coordinator.NotifyMuted().unwrap();
        } else {
            self.coordinator.NotifyUnmuted().unwrap();
        }

        call.NotifyCallActive()
            .expect("Call should be startable");
        self.call = Some(call);
        locked_state.is_in_call = true;
    }

    fn on_server_disconnected(&mut self, _conn: m::ConnectionT) {
        let locked_state = &mut self.state.lock().unwrap();
        // let api = &mut self.state.lock().unwrap().api;
        debug_log(&mut locked_state.api, "Server disconnected").unwrap();
        locked_state.is_in_call = false;

        if let Some(call) = self.call.as_ref() {
            call.NotifyCallEnded().expect("We should be able to end the call");
            self.call = None;
        }
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
            is_in_call: false,
        }));

        {
            // Useful for debugging.
            let state_ref = &mut state.lock().unwrap();
            debug_log(&mut state_ref.api, "Hello there!").unwrap();
        }

        {
            let state_copy = state.clone();
            coordinator.MuteStateChanged(&TypedEventHandler::new(move |_, args: &Option<MuteChangeEventArgs>| {
                if let Some(a) = args {
                    let locked_state = &mut state_copy.lock().unwrap();
                    let should_mute = a.Muted().unwrap();
                    debug_log(&mut locked_state.api, format!("Mute request - should mute? {}", should_mute).as_str()).unwrap();
                    if locked_state.is_in_call {
                        locked_state.api.request_local_user_mute(should_mute).unwrap();
                        if should_mute {
                            locked_state.coordinator.NotifyMuted().unwrap();
                        } else {
                            locked_state.coordinator.NotifyUnmuted().unwrap();
                        }
                    } else {
                        debug_log(&mut locked_state.api, "(ignoring since not in call)").unwrap();
                    }
                }
                Ok(())
            })).unwrap();
        }

        Ok(MutePlugin {
            state: state,
            coordinator: coordinator,
            call: None,
        })
    }
}

mumble_sys::register_mumble_plugin!(MutePlugin);