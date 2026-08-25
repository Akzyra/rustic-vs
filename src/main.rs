use freya::prelude::{LaunchConfig, WindowConfig, launch};

mod core;
mod ui;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new_app(ui::Rustic::new())
                .with_min_size(600.0, 450.0)
                .with_title("Rustic VS"),
        ),
    )
}
