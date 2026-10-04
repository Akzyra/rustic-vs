use crate::core::install::Install;
use crate::ui::components::*;
use freya::i18n::t;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Installs;

impl Component for Installs {
    fn render(&self) -> impl IntoElement {
        let installs = use_consume::<State<Vec<Install>>>();
        let mut show_add = use_state(|| false);

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
                    .child(
                        Button::new()
                            .on_press(move |_| show_add.set(true))
                            .child(t!("add_install")),
                    ),
            )
            .child(
                Table::new()
                    .column_widths([Size::px(100.0), Size::px(100.0), Size::px(100.0)])
                    .child(
                        TableRow::new()
                            .child(label().font_weight(FontWeight::BOLD).text(t!("install_id")))
                            .child(
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .text(t!("install_version")),
                            )
                            .child(label().font_weight(FontWeight::BOLD).text(t!("actions"))),
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
            .child(AddInstallPopup { show: show_add })
    }
}
#[derive(PartialEq)]
struct AddInstallPopup {
    show: State<bool>,
}

impl Component for AddInstallPopup {
    fn render(&self) -> impl IntoElement {
        let mut open = self.show;
        let mut id = use_state(String::new);
        let version = use_state(|| "latest".to_string());
        let mut installs = use_consume::<State<Vec<Install>>>();

        //TODO: read global cache of game version using API -> select box

        let progress_download = use_state(|| -1.0f32);
        let progress_install = use_state(|| -1.0f32);

        let on_close: EventHandler<()> = (move |_| {
            open.set(false);
            id.set(String::new());
        })
        .into();

        let on_submit: EventHandler<(String, String)> =
            (move |(raw_id, game_version): (String, String)| {
                //TODO: display error in UI, show if name cannot be used -> input on_validation
                let new_id = raw_id.trim().to_string();
                if new_id.is_empty() {
                    return;
                }

                let install = Install::new(new_id, Some(game_version));
                if installs.peek().iter().any(|i| i.id == install.id) {
                    log::warn!(
                        "install `{}` already exists, not adding a duplicate",
                        install.id
                    );
                    return;
                }

                // TODO: call download and installer, handle errors

                installs.write().push(install);

                // reset
                open.set(false);
                id.set(String::new());
                //TODO: reset select
            })
            .into();

        Popup::new()
            .width(Size::px(450.0))
            .on_close_request(on_close.clone())
            .maybe(open(), |popup| {
                popup.child(PopupTitle::new(t!("add_install"))).child(
                    PopupContent::new()
                        .child(
                            flex_rect(Direction::Vertical, SPACING_MD)
                                .child(form_row(
                                    t!("install_id"),
                                    Input::new(id)
                                        .width(Size::flex(1.0))
                                        .auto_focus(true)
                                        .placeholder(t!("install_id")),
                                ))
                                .child(form_row(
                                    t!("install_version"),
                                    // TOOD: add version select
                                    label().text(version.read().clone()),
                                ))
                                .maybe_child(progress_download.peek().is_sign_positive().then(
                                    || {
                                        ProgressBar::new(*progress_download.read())
                                            .width(Size::flex(1.0))
                                    },
                                ))
                                .maybe_child(progress_install.peek().is_sign_positive().then(
                                    || {
                                        ProgressBar::new(*progress_install.read())
                                            .width(Size::flex(1.0))
                                    },
                                ))
                                .child(label().overline().text(
                                    "Mockup, does not download new versions, in-memory only!",
                                )),
                        )
                        .child(
                            PopupButtons::new()
                                .child(
                                    Button::new()
                                        .on_press(move |_| on_close.call(()))
                                        .child(t!("cancel")),
                                )
                                .child(
                                    Button::new()
                                        .filled()
                                        .on_press(move |_| {
                                            let id = id.peek().clone();
                                            let version = version.peek().clone();
                                            on_submit.call((id, version));
                                        })
                                        .child(t!("create")),
                                ),
                        ),
                )
            })
    }
}
