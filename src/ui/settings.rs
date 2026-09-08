use crate::core::settings::THEMES;
use crate::ui::components::*;
use crate::ui::get_theme;
use freya::i18n::t;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Settings;

fn settings_row(lbl: String, elem: impl IntoElement) -> Rect {
    flex_rect(Direction::Horizontal, SPACING_SM)
        .cross_align(Alignment::Center)
        .child(label().width(Size::flex(2.0)).text(lbl))
        .child(rect().width(Size::px(200.0)).child(elem))
}

impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        let mut settings = use_consume::<State<crate::core::settings::Settings>>();
        let mut theme = use_theme();

        let current_name = theme.read().name;

        main_rect(t!("settings_header")).child(settings_row(
            t!("settings_theme"),
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
        ))
    }
}
