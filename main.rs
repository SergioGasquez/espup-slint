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

    let xtensa_version = app.global::<Espup>().get_xtensa_rust_version();
    println!("Xtensa Version: {}", xtensa_version);
    app.global::<Espup>()
        .set_xtensa_rust_version("aaaaaa0".into());
    println!("Xtensa Version: {}", xtensa_version);

    app.run();
}
