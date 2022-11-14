// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

#![deny(unsafe_code)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let app = App::new();
    app.set_espup_ui_version(env!("CARGO_PKG_VERSION").into());

    // Set default
    // println!(
    //     "Xtensa Version: {}",
    //     app.global::<Espup>().get_xtensa_rust_version()
    // );
    app.global::<Espup>()
        .set_xtensa_rust_version("1.65.0.1".into());

    // Button callback
    app.global::<Espup>().on_install({
        let ui_handle = app.as_weak();
        move || {
            println!("Install button clicked");
            // println!("Button pressed");
            let ui = ui_handle.unwrap();
            let esp32 = ui.global::<Espup>().get_esp32_value();
            println!("ESP32: {}", esp32);
        }
    });

    app.global::<Espup>().on_esp32({
        let ui_handle = app.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let esp32 = ui.global::<Espup>().get_esp32_value();
            ui.global::<Espup>().set_esp32_value(!esp32);
            println!("Esp32 updated: {}", !esp32);
        }
    });

    app.run();
}
