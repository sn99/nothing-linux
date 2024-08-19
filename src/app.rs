use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (address, set_address) = create_signal("Unknown".to_string());
    let (serial_number, set_serial_number) = create_signal("Unknown".to_string());
    let (firmware_version, set_firmware_version) = create_signal("Unknown".to_string());

    // view! {
    //     <div class="button-container">
    //         <Info
    //             read_signal=address
    //             write_signal=set_address
    //             function_name="get_address"
    //             name="Address"
    //         />
    //         <Info
    //             read_signal=serial_number
    //             write_signal=set_serial_number
    //             function_name="get_serial_number"
    //             name="Serial Number"
    //         />
    //         <Info
    //             read_signal=firmware_version
    //             write_signal=set_firmware_version
    //             function_name="get_firmware_version"
    //             name="Firmware Version"
    //         />
    //     </div>
    //
    //     <button
    //         on:click=move |_| {
    //             set_count.update(|n| *n += 1);
    //         }
    //         class:red=move || count.get() % 2 == 1
    //     >
    //         "Click me for +1"
    //     </button>
    //
    //     <p>
    //         <strong>"Reactive: "</strong>
    //         {move || count.get()}
    //     </p>
    // }
    view! {
        <div class="container">
            <div class="row">
                <div class="column noise-control">
                    <h2>"NOISE CONTROL"</h2>
                    <div class="toggle-group" id="noiseControl">
                        <div class="active" data-noise="cancellation">
                            "Noise Cancellation"
                        </div>
                        <div data-noise="transparency">"Transparency"</div>
                        <div data-noise="off">"Off"</div>
                    </div>
                    <div class="noise-levels" id="noiseLevels">
                        <div>"High"</div>
                        <div>"Mid"</div>
                        <div>"Low"</div>
                        <div class="active">"Adaptive"</div>
                    </div>
                </div>
                <div class="column">
                    <div class="switch-group">
                        <span class="switch-label">"Personalized ANC"</span>
                        <label class="switch">
                            <input type="checkbox" id="togglePersonalizedANC" />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="switch-group">
                        <span class="switch-label">"Low Lag Mode"</span>
                        <label class="switch">
                            <input type="checkbox" id="toggleLowLagMode" />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="switch-group">
                        <span class="switch-label">"High Quality Audio"</span>
                        <label class="switch">
                            <input type="checkbox" id="toggleHighQualityAudio" />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="switch-group">
                        <span class="switch-label">"In-Ear Detection"</span>
                        <label class="switch">
                            <input type="checkbox" id="toggleInEarDetection" />
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>
            </div>
            <div class="info">
                <p>"Version: 1.0.0 | Firmware: 2.1.0 | Address: 1234 Street Name, City"</p>
            </div>
        </div>
    }
}

#[component]
pub fn Info(
    read_signal: ReadSignal<String>,
    write_signal: WriteSignal<String>,
    function_name: &'static str,
    name: &'static str,
) -> impl IntoView {
    view! {
        <div class="button-paragraph-container">
            // Get Address
            <button on:click=move |_| {
                spawn_local(async move {
                    let address = invoke(function_name).await.as_string().unwrap();
                    write_signal.update(move |n| *n = address);
                });
            }>{format!("Get {}", name)}</button>
            <p>
                <strong>{format!("{}: ", name)}</strong>
                {move || read_signal.get()}
            </p>
        </div>
    }
}
