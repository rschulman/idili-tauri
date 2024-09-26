use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
struct Event {
    id: u32,
    event: String,
    payload: EventPayload,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum EventPayload {
    VeilidConnected(AttachEvent),
}

#[derive(Serialize, Deserialize, Clone)]
struct AttachEvent {
    state: VeilidAttachmentState,
    public_internet_ready: bool,
    local_network_ready: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum VeilidAttachmentState {
    Detached = 0,
    Attaching = 1,
    AttachedWeak = 2,
    AttachedGood = 3,
    AttachedStrong = 4,
    FullyAttached = 5,
    OverAttached = 6,
    Detaching = 7,
}

impl std::fmt::Display for VeilidAttachmentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let out = match self {
            VeilidAttachmentState::Attaching => "Attaching".to_owned(),
            VeilidAttachmentState::AttachedWeak => "Attached Weak".to_owned(),
            VeilidAttachmentState::AttachedGood => "Attached Good".to_owned(),
            VeilidAttachmentState::AttachedStrong => "Attached Strong".to_owned(),
            VeilidAttachmentState::FullyAttached => "Fully Attached".to_owned(),
            VeilidAttachmentState::OverAttached => "Over Attached".to_owned(),
            VeilidAttachmentState::Detaching => "Detaching".to_owned(),
            VeilidAttachmentState::Detached => "Detached".to_owned(),
        };
        write!(f, "{}", out)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "event"])]
    pub async fn listen(
        event: &str,
        closure: &Closure<dyn Fn(JsValue)>,
    ) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct LookupArgs<'a> {
    address: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (connected, set_connected) = create_signal(None);
    let (dht_address, set_dht_address) = create_signal("".to_string());
    let (dht_content, set_dht_content) = create_signal("".to_string());

    let lookup = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let address = dht_address.get_untracked();
            if address.is_empty() {
                return;
            }
            let args = serde_wasm_bindgen::to_value(&LookupArgs { address: &address }).unwrap();

            let new_content = invoke("lookup", args).await.unwrap().as_string().unwrap();
            set_dht_content.set(new_content);
        })
    };

    let update_address = move |ev| {
        let v = event_target_value(&ev);
        set_dht_address.set(v);
    };

    create_local_resource(
        || (),
        move |_| async move {
            let cb = Closure::<dyn Fn(JsValue)>::new(move |e: JsValue| {
                log::info!("Veilid State: {e:?}");
                let ev = serde_wasm_bindgen::from_value::<Event>(e).unwrap();
                set_connected.set(Some(ev));
            });
            listen("veilid_connected", &cb)
                .await
                .expect("Failed to create listener");
            cb.forget();
        },
    );

    view! {
        <main class="container">
            <div>Veilid is
             { move || {
                 if let Some(s) = connected.get() {
                     match s.payload {
                         EventPayload::VeilidConnected(conn) => conn.state.to_string(),
                     }
                 } else {
                     "... unknown!".to_string()
                 }
               }
            }
            </div>
            <div>
            <form on:submit=lookup>
                <input
                    id="address-input"
                    placeholder="Enter a DHT address"
                    on:input=update_address
                />
                <button type="submit">Lookup</button>
            </form>
            </div>

            <div>{move || dht_content.get() } </div>
        </main>
    }
}
