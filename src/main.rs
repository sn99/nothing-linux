pub mod app;

use app::App;
use leptos::*;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
