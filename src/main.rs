// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial
#![deny(unsafe_code)]

use std::collections::HashSet;

use espup::targets::Target;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let mut targets: HashSet<Target> = HashSet::new();
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
            let ui = ui_handle.unwrap();
            // Get targets
            targets.clear();
            if ui.global::<Espup>().get_esp32_value() {
                targets.insert(Target::ESP32);
            }
            if ui.global::<Espup>().get_esp32s2_value() {
                targets.insert(Target::ESP32S2);
            }
            if ui.global::<Espup>().get_esp32s3_value() {
                targets.insert(Target::ESP32S3);
            }
            if ui.global::<Espup>().get_esp32c2_value() {
                targets.insert(Target::ESP32C2);
            }
            if ui.global::<Espup>().get_esp32c3_value() {
                targets.insert(Target::ESP32C3);
            }
            println!("Targets: {:#?}", targets);
        }
    });

    app.run();
}
