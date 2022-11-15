#![deny(unsafe_code)]

use anyhow::Result;
use std::{collections::HashSet, path::PathBuf};

use espup::{
    host_triple::get_host_triple, install, targets::Target, toolchain::rust::Crate,
    toolchain::rust::XtensaRust, uninstall, update, InstallOpts, UninstallOpts, UpdateOpts,
};

// TODO: Get this from the espup
#[cfg(windows)]
const DEFAULT_EXPORT_FILE: &str = "export-esp.ps1";
#[cfg(not(windows))]
const DEFAULT_EXPORT_FILE: &str = "export-esp.sh";

slint::include_modules!();

pub fn main() -> Result<()> {
    let host_triple = get_host_triple(None)?;
    let latest_xtensa_rust = XtensaRust::get_latest_version()?;

    let app = App::new();
    app.set_espup_ui_version(env!("CARGO_PKG_VERSION").into());

    // Set defaults
    // Install
    app.global::<InstallArgs>()
        .set_xtensa_rust_version(latest_xtensa_rust.clone().into());
    app.global::<InstallArgs>()
        .set_default_host(host_triple.to_string().into());
    app.global::<InstallArgs>()
        .set_export_file(DEFAULT_EXPORT_FILE.into());
    // Update
    app.global::<UpdateArgs>()
        .set_xtensa_rust_version(latest_xtensa_rust.into());
    app.global::<UpdateArgs>()
        .set_default_host(host_triple.to_string().into());

    // Install callback
    app.global::<InstallArgs>().on_install({
        let ui_handle = app.as_weak();
        move || {
            println!("Install button clicked");
            let mut selected_crates: HashSet<Crate> = HashSet::new();
            let mut targets: HashSet<Target> = HashSet::new();
            let ui = ui_handle.unwrap();

            // Get targets
            if ui.global::<InstallArgs>().get_esp32_value() {
                targets.insert(Target::ESP32);
            }
            if ui.global::<InstallArgs>().get_esp32s2_value() {
                targets.insert(Target::ESP32S2);
            }
            if ui.global::<InstallArgs>().get_esp32s3_value() {
                targets.insert(Target::ESP32S3);
            }
            if ui.global::<InstallArgs>().get_esp32c2_value() {
                targets.insert(Target::ESP32C2);
            }
            if ui.global::<InstallArgs>().get_esp32c3_value() {
                targets.insert(Target::ESP32C3);
            }

            // Get extra crates
            if ui.global::<InstallArgs>().get_espflash_value() {
                selected_crates.insert(Crate::new("espflash"));
            }
            if ui.global::<InstallArgs>().get_cargo_espflash_value() {
                selected_crates.insert(Crate::new("cargo-espflash"));
            }
            if ui.global::<InstallArgs>().get_cargo_generate_value() {
                selected_crates.insert(Crate::new("cargo-generate"));
            }
            if ui.global::<InstallArgs>().get_ldproxy_value() {
                selected_crates.insert(Crate::new("ldproxy"));
            }
            if ui.global::<InstallArgs>().get_sccache_value() {
                selected_crates.insert(Crate::new("sccache"));
            }
            let extra_crates = if selected_crates.is_empty() {
                None
            } else {
                Some(selected_crates)
            };

            // Host triple
            let host_triple = ui.global::<InstallArgs>().get_default_host();

            // Log Level
            let log_level = ui.global::<InstallArgs>().get_log_level().to_string();

            // Export file
            let export_file = ui.global::<InstallArgs>().get_export_file();
            let export_file = Some(PathBuf::from(export_file.as_str()));

            // ESP-IDF version
            let esp_idf_version = if (ui.global::<InstallArgs>().get_esp_idf_version()) == "none" {
                None
            } else {
                Some(ui.global::<InstallArgs>().get_esp_idf_version().to_string())
            };

            // Xtensa Rust Toolhain version
            let xtensa_rust_version = ui
                .global::<InstallArgs>()
                .get_xtensa_rust_version()
                .to_string();

            // Nightly Rust Toolhain version
            let nightly_version = ui.global::<InstallArgs>().get_nightly_version().to_string();

            let profile_minimal = ui.global::<InstallArgs>().get_profile_minimal();

            let opts = InstallOpts {
                default_host: Some(host_triple.into()),
                esp_idf_version,
                export_file,
                extra_crates,
                llvm_version: "15".into(),
                log_level,
                nightly_version,
                profile_minimal,
                targets: targets.clone(),
                toolchain_version: Some(xtensa_rust_version),
            };
            println!("Install options: {:#?}", opts);
            install(opts);
        }
    });

    // Update callback
    app.global::<UpdateArgs>().on_update({
        let ui_handle = app.as_weak();
        move || {
            println!("Update button clicked");
            let ui = ui_handle.unwrap();

            // Host triple
            let host_triple = ui.global::<UpdateArgs>().get_default_host();

            // Log Level
            let log_level = ui.global::<UpdateArgs>().get_log_level().to_string();

            // Xtensa Rust Toolhain version
            let xtensa_rust_version = ui
                .global::<UpdateArgs>()
                .get_xtensa_rust_version()
                .to_string();

            let opts = UpdateOpts {
                default_host: Some(host_triple.into()),
                log_level,
                toolchain_version: Some(xtensa_rust_version),
            };
            println!("Update options: {:#?}", opts);
            update(opts);
        }
    });

    // Uninstall callback
    app.global::<UninstallArgs>().on_uninstall({
        let ui_handle = app.as_weak();
        move || {
            println!("Uninstall button clicked");
            let ui = ui_handle.unwrap();

            // Log Level
            let log_level = ui.global::<UninstallArgs>().get_log_level().to_string();

            let opts = UninstallOpts { log_level };
            println!("Uninstall options: {:#?}", opts);
            uninstall(opts);
        }
    });

    app.run();
    Ok(())
}
