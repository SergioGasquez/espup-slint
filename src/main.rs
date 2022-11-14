// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial
#![deny(unsafe_code)]

use anyhow::Result;
use std::collections::HashSet;

use espup::{
    host_triple::get_host_triple, install, targets::Target, toolchain::rust::Crate,
    toolchain::rust::XtensaRust, InstallOpts,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() -> Result<()> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let mut targets: HashSet<Target> = HashSet::new();
    let host_triple = get_host_triple(None)?;
    let mut extra_crates: HashSet<Crate> = HashSet::new();
    let latest_xtensa_rust = XtensaRust::get_latest_version()?;

    let app = App::new();
    app.set_espup_ui_version(env!("CARGO_PKG_VERSION").into());

    // Set defaults
    app.global::<Espup>()
        .set_xtensa_rust_version(latest_xtensa_rust.into());
    app.global::<Espup>()
        .set_default_host(host_triple.to_string().into());

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

            // Get extra crates
            extra_crates.clear();
            if ui.global::<Espup>().get_espflash_value() {
                extra_crates.insert(Crate::new("espflash"));
            }
            if ui.global::<Espup>().get_cargo_espflash_value() {
                extra_crates.insert(Crate::new("cargo-espflash"));
            }
            if ui.global::<Espup>().get_ldproxy_value() {
                extra_crates.insert(Crate::new("ldproxy"));
            }
            if ui.global::<Espup>().get_sccache_value() {
                extra_crates.insert(Crate::new("sccache"));
            }
            println!("Extra crates: {:#?}", extra_crates);
        }
    });

    app.run();
    Ok(())
}
