use crate::core::settings::THEMES;
use crate::ui::components::*;
use crate::ui::get_theme;
use freya::i18n::tid;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Settings;

impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        let mut settings = use_consume::<State<crate::core::settings::Settings>>();
        let mut theme = use_theme();

        let current_name = theme.read().name;

        let label_width = Size::px(150.0);
        flex_rect(Direction::Vertical, SPACING_MD)
            .theme_background()
            .padding(SPACING_SM)
            .child(
                rect()
                    .content(Content::Flex)
                    .horizontal()
                    .spacing(SPACING_SM)
                    .child(
                        label()
                            .font_weight(FontWeight::BOLD)
                            .font_size(20.0)
                            .text(tid!("settings_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(label().font_family("Consolas").text(crate::NAME_VERSION)),
            )
            .child(
                flex_rect(Direction::Vertical, SPACING_SM)
                    .width(Size::px(450.0))
                    .child(form_row(
                        &label_width,
                        tid!("settings_theme"),
                        Select::new()
                            .width(Size::flex(1.0))
                            .selected_item(current_name)
                            .children(THEMES.iter().map(|t| {
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
                            })),
                    )),
            )
    }
}
