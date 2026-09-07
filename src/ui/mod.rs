use crate::core::settings::Theme;
use freya::components::{dark_theme, light_theme};

mod app;
mod components;
mod installs;
mod instance_edit;
mod instances;
mod news;
mod settings;

fn get_theme(theme: &Theme) -> freya::prelude::Theme {
    match theme {
        Theme::Light => light_theme(),
        Theme::Dark => dark_theme(),
    }
}

pub use crate::ui::app::app;
