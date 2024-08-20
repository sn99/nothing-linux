use leptos::logging::log;
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
    let (info, set_info) = create_signal("Unknown".to_string());

    spawn_local(async move {
        let address = invoke("get_address").await.as_string().unwrap();
        let serial_number = invoke("get_serial_number").await.as_string().unwrap();
        let firmware_version = invoke("get_firmware_version").await.as_string().unwrap();

        set_info.set(format!(
            "Address: {} | Serial Number: {} | Firmware Version: {}",
            address, serial_number, firmware_version
        ));
    });

    let (noise_control, set_noise_control) = create_signal("cancellation".to_string());
    let (noise_level, set_noise_level) = create_signal("adaptive".to_string());

    let toggle_noise_control = move |mode: &str| {
        set_noise_control.set(mode.to_string());
    };

    let toggle_noise_level = move |level: &str| {
        set_noise_level.set(level.to_string());
    };

    let handle_low_lag_mode = move |event| {
        spawn_local(async move {
            let is_checked = event_target_checked(&event);
            log!("Low Lag Mode: {}", is_checked);
            if is_checked {
                invoke("set_low_lag_mode_on").await;
            } else {
                invoke("set_low_lag_mode_off").await;
            }
        });
    };

    let handle_in_ear_detection_mode = move |event| {
        spawn_local(async move {
            let is_checked = event_target_checked(&event);
            log!("In-Ear Detection Mode: {}", is_checked);
            if is_checked {
                invoke("set_in_ear_detection_on").await;
            } else {
                invoke("set_in_ear_detection_off").await;
            }
        });
    };

    view! {
        <div class="container">
            <div class="row">
                <div class="column noise-control">
                    <h2>"NOISE CONTROL"</h2>
                    <div class="toggle-group">
                        <div
                            class=move || {
                                if noise_control.get() == "cancellation" { "active" } else { "" }
                            }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_adaptive").await;
                                    toggle_noise_control("cancellation")
                                });
                            }
                        >
                            "Noise Cancellation"
                        </div>
                        <div
                            class=move || {
                                if noise_control.get() == "transparency" { "active" } else { "" }
                            }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_transparency").await;
                                    toggle_noise_control("transparency")
                                });
                            }
                        >
                            "Transparency"
                        </div>
                        <div
                            class=move || if noise_control.get() == "off" { "active" } else { "" }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_off").await;
                                    toggle_noise_control("off")
                                });
                            }
                        >
                            "Off"
                        </div>
                    </div>

                    <div class=move || {
                        if noise_control.get() == "cancellation" {
                            "noise-levels"
                        } else {
                            "noise-levels hidden"
                        }
                    }>
                        <div
                            class=move || if noise_level.get() == "high" { "active" } else { "" }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_high").await;
                                    toggle_noise_level("high")
                                });
                            }
                        >
                            "High"
                        </div>
                        <div
                            class=move || if noise_level.get() == "mid" { "active" } else { "" }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_mid").await;
                                    toggle_noise_level("mid")
                                });
                            }
                        >
                            "Mid"
                        </div>
                        <div
                            class=move || if noise_level.get() == "low" { "active" } else { "" }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_low").await;
                                    toggle_noise_level("low")
                                });
                            }
                        >
                            "Low"
                        </div>
                        <div
                            class=move || {
                                if noise_level.get() == "adaptive" { "active" } else { "" }
                            }
                            on:click=move |_| {
                                spawn_local(async move {
                                    invoke("set_anc_mode_adaptive").await;
                                    toggle_noise_level("adaptive")
                                });
                            }
                        >
                            "Adaptive"
                        </div>
                    </div>
                </div>

                <div class="column">
                    <div class="switch-group">
                        <span class="switch-label">"Low Lag Mode"</span>
                        <label class="switch">
                            <input type="checkbox" on:change=handle_low_lag_mode />
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="switch-group">
                        <span class="switch-label">"In-Ear Detection"</span>
                        <label class="switch">
                            <input type="checkbox" on:change=handle_in_ear_detection_mode />
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>
            </div>

            <div class="info">
                <p>{move || info.get()}</p>
            </div>
        </div>
    }
}
