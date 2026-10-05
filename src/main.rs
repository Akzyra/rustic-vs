use freya::prelude::{LaunchConfig, WindowConfig, launch};
mod core;
mod ui;

static NAME_VERSION: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
static USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/Akzyra/rustic-vs)",
);

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
