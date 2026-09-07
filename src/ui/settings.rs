use crate::core::settings::THEMES;
use crate::ui::components::*;
use crate::ui::get_theme;
use freya::i18n::t;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Settings;

impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        let mut settings = use_consume::<State<crate::core::settings::Settings>>();
        let mut theme = use_theme();

        let current_name = theme.read().name;

        main_rect(t!("settings_header")).child(
            rect().child(t!("settings_theme")).child(
                Select::new()
                    .width(Size::flex(1.0))
                    .selected_item(current_name)
                    .children(THEMES.into_iter().map(|t| {
                        let ft = get_theme(&t.clone());
                        let name = ft.name;

                        MenuItem::new()
                            .selected(name == current_name)
                            .on_press(move |_| {
                                let t = t.clone();
                                theme.set(get_theme(&t));
                                settings.write().theme = t;
                                settings.read().save();
                            })
                            .child(name)
                            .into()
                    })),
            ),
        )
    }
}
