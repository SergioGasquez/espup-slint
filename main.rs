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
    let mut esp32: bool = false;
    app.set_espup_ui_version(env!("CARGO_PKG_VERSION").into());

    // Set default
    // println!(
    //     "Xtensa Version: {}",
    //     app.global::<Espup>().get_xtensa_rust_version()
    // );
    app.global::<Espup>()
        .set_xtensa_rust_version("1.65.0.1".into());

    app.global::<Espup>().on_esp32(move || {
        esp32 = !esp32;
        println!("Esp32 updated: {}", esp32);
    });

    // Button callback
    // let ui_handle = app.as_weak();
    app.global::<Espup>().on_install(move || {
        println!("Install clicked");
        // println!("Button pressed");
        // let ui = ui_handle.unwrap();
        // println!("ESP32: {}", ui.get_esp32());
        // let esp32 = ui.global::<InstallPage>().get_esp32();
        // println!("Esp32: {}", ui.global::<Espup>().get_esp32());
        // println!("Esp32: {}", esp32);
    });

    app.run();
}
