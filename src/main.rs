use freya::prelude::{LaunchConfig, WindowConfig, launch};
mod core;
mod ui;

#[tokio::main]
async fn main() -> Result<(), fern::InitError> {
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
        .chain(std::io::stdout())
        .chain(fern::log_file("rustic.log")?)
        .apply()?;

    Ok(launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(ui::app)
                .with_min_size(750.0, 500.0)
                .with_title("Rustic VS"),
        ),
    ))
}
