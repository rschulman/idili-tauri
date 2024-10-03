use leptos::prelude::*;
use leptos::spawn::spawn_local;
use leptos::web_sys::SubmitEvent;
use serde::{Deserialize, Serialize};
use thaw::*;
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
    VeilidConnected(bool),
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
    let (connected, set_connected) = signal(false);
    let (dht_address, set_dht_address) = signal("".to_string());
    let (dht_content, set_dht_content) = signal("".to_string());

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

    let cb = Closure::<dyn Fn(JsValue)>::new(move |e: JsValue| {
        log::info!("Veilid State: {e:?}");
        let ev = serde_wasm_bindgen::from_value::<Event>(e).unwrap();
        match ev.payload {
            EventPayload::VeilidConnected(c) => set_connected(c),
        }
    });

    spawn_local(async move {
        let _ = listen("veilid_connected", &cb).await;
        cb.forget();
    });

    view! {
        <main class="container">
            <Layout>
                <LayoutHeader class="bg-emerald-50">
                    <Flex justify=FlexJustify::SpaceBetween>
                        <Icon icon=icondata::AiAlignLeftOutlined class="m-2" width="36" height="36" />
                        "Idili"
                        <Show
                            when=move || { connected() }
                            fallback=|| view! { <Icon icon=icondata::TbLinkOff class="m-2" width="36" height="36" /> }
                        >
                            <Icon icon=icondata::TbLink class="m-2" width="36" height="36" />
                        </Show>
                    </Flex>
                </LayoutHeader>
                <Layout>
                    <Flex justify=FlexJustify::Center>
                    <form on:submit=lookup>
                        <Field
                            label="DHT Lookup"
                            name="address-input"
                            on:input=update_address
                        >
                            <Input />
                        </Field>
                        <Button button_type=ButtonType::Submit>
                            "Submit"
                        </Button>
                    </form>
                    </Flex>
                </Layout>
            </Layout>

            <div>{move || dht_content.get() } </div>
        </main>
    }
}
