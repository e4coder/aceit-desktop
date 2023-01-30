use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct EncryptArgs<'a> {
    text: &'a str,
    key: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let key_input_ref = use_node_ref();
    let text_input_ref = use_node_ref();
    let text_output_ref = use_node_ref();

    let key = use_state(|| String::new());
    let text = use_state(|| String::new());
    let out_text = use_state(|| String::new());

    let handle_change_text = {
        let text = text.clone();
        let text_input_ref = text_input_ref.clone();
        Callback::from(move |_e: InputEvent| {
            text.set(text_input_ref.cast::<web_sys::HtmlTextAreaElement>().unwrap().value());
        })
    };
    let handle_change_key = {
        let key = key.clone();
        let key_input_ref = key_input_ref.clone();
        Callback::from(move |_e: InputEvent| {
            key.set(key_input_ref.cast::<web_sys::HtmlTextAreaElement>().unwrap().value());
        })
    };

    {
        let out = out_text.clone();
        let out2 = out_text.clone();
        let text_out_ref = text_output_ref.clone();
        use_effect_with_deps(
            move |_| {
                   text_out_ref.cast::<web_sys::HtmlTextAreaElement>().unwrap().set_value(&out); 
                || {}
            },
            out2,
        );
    }

    fn process (typ: u8, text: UseStateHandle<String>, out: UseStateHandle<String>, key: UseStateHandle<String>) {
        let text = text.clone();
        let key = key.clone();
        let out = out.clone();
        spawn_local(async move {
            let mut invoke_name = "";
            if typ == 1 {
                invoke_name = "encrypt";
            } else if typ == 2 {
                invoke_name = "decrypt";
            }

            let new_msg = invoke(
                invoke_name,
                to_value(&EncryptArgs {
                    text: &*text,
                    key: &*key,
                })
                .unwrap(),
            )
            .await;
            out.set(new_msg.as_string().unwrap());
        });        
    }

    let decrypt = {
        let text = text.clone();
        let key = key.clone();
        let out = out_text.clone();
        Callback::from(move |_| {
            let text = text.clone();
            let key = key.clone();
            let out = out.clone();

            process(2, text, out, key);
        })
    };
    let encrypt = {
        let text = text.clone();
        let key = key.clone();
        let out = out_text.clone();
        Callback::from(move |_| {
            let text = text.clone();
            let key = key.clone();
            let out = out.clone();

            process(1, text, out, key);
        })
    };
    
    html! {
        <main class="app">
            <div class="container">
                <div class="row">
                    <textarea oninput={handle_change_text} ref={text_input_ref} placeholder="Input..." />
                </div>
                <div class="row">
                    <textarea readonly={true} ref={text_output_ref} placeholder="Output..." />
                </div>
                <div class="row">
                    <input id="key-input" oninput={handle_change_key} ref={key_input_ref} placeholder="Key..." />
                </div>
                <div class="row-bottom">
                    <button type="button" onclick={encrypt}>{"Encrypt"}</button>
                    <button type="button" onclick={decrypt}>{"Decrypt"}</button>
                </div>
            </div>
        </main>
    }
}
