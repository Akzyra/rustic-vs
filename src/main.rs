use freya::prelude::{LaunchConfig, WindowConfig, launch};

mod core;
mod ui;

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
        .chain(std::io::stdout())
        .chain(fern::log_file("rustic.log")?)
        .apply()?;

    let cwd = std::env::current_dir().expect("how can there be no CWD");
    let mut app = ui::Rustic::new(cwd);
    app.reload();

    Ok(launch(
        LaunchConfig::new().with_window(
            WindowConfig::new_app(app)
                .with_min_size(600.0, 450.0)
                .with_title("Rustic VS"),
        ),
    ))
}
