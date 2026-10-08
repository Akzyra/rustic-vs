#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::vs::api::DownloadPlatform;
use freya::prelude::{LaunchConfig, WindowConfig, launch};
use tokio::runtime::Builder;

mod core;
mod ui;
mod vs;

static NAME: &str = env!("CARGO_PKG_NAME");
static NAME_VERSION: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
static USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/Akzyra/rustic-vs)",
);

#[rustfmt::skip] 
const PLATFORM: DownloadPlatform = {
    #[cfg(target_os = "windows")]
    { DownloadPlatform::Windows }
    #[cfg(target_os = "linux")]
    { DownloadPlatform::Linux }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    { DownloadPlatform::MacArm64 }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    { DownloadPlatform::MacX64 }
};
const ICON: &[u8] = include_bytes!("../assets/temp.png");

fn main() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("html5ever", log::LevelFilter::Error)
        .level_for("style", log::LevelFilter::Error)
        .level_for("selectors", log::LevelFilter::Error)
        .level_for("rustls", log::LevelFilter::Error)
        .level_for("reqwest", log::LevelFilter::Error)
        .level_for("ureq", log::LevelFilter::Error)
        .level_for("ureq_proto", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .chain(fern::log_file("rustic.log")?)
        .apply()?;

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let _rt = rt.enter();

    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(ui::app)
                .with_min_size(750.0, 500.0)
                .with_icon(LaunchConfig::window_icon(ICON))
                .with_title("Rustic VS"),
        ),
    );

    Ok(())
}
