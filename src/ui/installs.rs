use crate::core::install::Install;
use crate::ui::components::*;
use freya::i18n::t;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Installs;

impl Component for Installs {
    fn render(&self) -> impl IntoElement {
        let installs = use_consume::<State<Vec<Install>>>();

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
                            .text(t!("installs_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(Button::new().child(t!("add_install"))),
            )
            .child(
                Table::new()
                    .column_widths([Size::px(100.0), Size::px(100.0), Size::px(100.0)])
                    .child(
                        TableRow::new()
                            .child(
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .text(t!("installs_column_id")),
                            )
                            .child(
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .text(t!("installs_column_version")),
                            )
                            .child(
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .text(t!("installs_column_actions")),
                            ),
                    )
                    .child(
                        ScrollView::new().children(installs.read().iter().enumerate().map(
                            |(i, install)| {
                                TableRow::new()
                                    .key(i)
                                    .child(install.id.clone())
                                    .child(
                                        install
                                            .game_version
                                            .clone()
                                            .unwrap_or_else(|| "—".to_string()),
                                    )
                                    .child("todo")
                            },
                        )),
                    ),
            )
    }
}
