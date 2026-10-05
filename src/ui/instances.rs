use crate::core::install::Install;
use crate::core::instance::Instance;
use crate::ui::components::*;
use freya::i18n::t;
use freya::icons::lucide;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Instances;

impl Component for Instances {
    fn render(&self) -> impl IntoElement {
        let instances = use_consume::<State<Vec<Instance>>>();
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
                            .text(t!("instances_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(
                        Button::new()
                            .on_press(move |_| show_add.set(true))
                            .child(t!("add_instance")),
                    ),
            )
            .child(
                Table::new()
                    .column_widths([
                        Size::px(100.0),
                        Size::flex(1.0),
                        Size::px(100.0),
                        Size::px(100.0),
                    ])
                    .child(
                        TableRow::new()
                            .child(
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .text(t!("instance_id")),
                            )
                            .child(
                                label()
                                    .font_weight(FontWeight::BOLD)
                                    .text(t!("instance_name")),
                            )
                            .child(label().font_weight(FontWeight::BOLD).text(t!("install_id")))
                            .child(label().font_weight(FontWeight::BOLD).text(t!("actions"))),
                    )
                    .child(
                        ScrollView::new().children(instances.read().iter().enumerate().map(
                            |(i, instance)| {
                                TableRow::new()
                                    .key(i)
                                    .child(instance.id.clone())
                                    .child(instance.name.clone())
                                    .child(
                                        instance
                                            .install_id
                                            .clone()
                                            .unwrap_or_else(|| "—".to_string()),
                                    )
                                    .child("todo")
                            },
                        )),
                    ),
            )
            .child(AddInstancePopup { show: show_add })
    }
}

#[derive(PartialEq)]
struct AddInstancePopup {
    show: State<bool>,
}

impl Component for AddInstancePopup {
    fn render(&self) -> impl IntoElement {
        let mut open = self.show;
        let mut name = use_state(String::new);
        let mut instances = use_consume::<State<Vec<Instance>>>();

        let installs = use_consume::<State<Vec<Install>>>();
        let mut selected_install_idx: State<Option<usize>> = use_state(|| None);

        let on_close: EventHandler<()> = (move |_| {
            open.set(false);
            name.set(String::new());
            selected_install_idx.set(None);
        })
        .into();

        let on_submit: EventHandler<(String, Option<String>)> =
            (move |(raw_name, install_id): (String, Option<String>)| {
                //TODO: display error in UI, show if name cannot be used -> input on_validation
                let new_name = raw_name.trim().to_string();
                if new_name.is_empty() {
                    return;
                }

                let instance = Instance::new(new_name, install_id);
                if instances.peek().iter().any(|i| i.id == instance.id) {
                    log::warn!(
                        "instance `{}` already exists, not adding a duplicate",
                        instance.id
                    );
                    return;
                }

                let Ok(cwd) = std::env::current_dir() else {
                    log::error!("failed to get current dir, cannot save instance");
                    return;
                };

                if let Err(e) = instance.save_to_dir(&cwd) {
                    log::error!("failed to save instance `{}`: {}", instance.id, e);
                    return;
                }

                instances.write().push(instance);

                // reset
                open.set(false);
                name.set(String::new());
                selected_install_idx.set(None);
            })
            .into();

        Popup::new()
            .width(Size::px(450.0))
            .on_close_request(on_close.clone())
            .maybe(open(), |popup| {
                popup
                    .child(PopupTitle::new(t!("add_instance_title")))
                    .child(
                        PopupContent::new().child(
                            flex_rect(Direction::Vertical, SPACING_MD)
                                .child(form_row(
                                    t!("instance_name"),
                                    Input::new(name)
                                        .width(Size::flex(1.0))
                                        .auto_focus(true)
                                        .placeholder(t!("instance_name")),
                                ))
                                .child(form_row(
                                    t!("install_id"),
                                    flex_rect(Direction::Horizontal, SPACING_SM)
                                        .width(Size::flex(1.0))
                                        .child(
                                            // need a proper flex that shinks for the X button
                                            rect().width(Size::flex(1.0)).child(
                                                Select::new()
                                                    // fill the parent fully
                                                    .width(Size::fill())
                                                    .selected_item(
                                                        selected_install_idx.read().map_or(
                                                            "--".into_element(),
                                                            |i| {
                                                                installs.peek().get(i).map_or(
                                                                    "---".into_element(),
                                                                    |si| {
                                                                        install_widget(si)
                                                                            .into_element()
                                                                    },
                                                                )
                                                            },
                                                        ),
                                                    )
                                                    .children(
                                                        installs.read().iter().enumerate().map(
                                                            |(i, install)| {
                                                                MenuItem::new()
                                                                    .key(i)
                                                                    .selected(
                                                                        selected_install_idx()
                                                                            == Some(i),
                                                                    )
                                                                    .on_press(move |_| {
                                                                        selected_install_idx
                                                                            .set(Some(i));
                                                                    })
                                                                    .child(install_widget(install))
                                                            },
                                                        ),
                                                    ),
                                            ),
                                        )
                                        .child(
                                            Button::new()
                                                .width(Size::auto())
                                                .padding(6.0)
                                                .outline()
                                                .on_press(move |_| selected_install_idx.set(None))
                                                .child(
                                                    SvgViewer::new(lucide::x())
                                                        .width(Size::px(24.0))
                                                        .height(Size::px(24.0)),
                                                ),
                                        ),
                                )),
                        ),
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
                                        let current_name = name.peek().clone();
                                        let install_id =
                                            selected_install_idx.read().map_or(None, |i| {
                                                installs
                                                    .peek()
                                                    .get(i)
                                                    .map(|install| install.id.clone())
                                            });
                                        on_submit.call((current_name, install_id));
                                    })
                                    .child(t!("create")),
                            ),
                    )
            })
    }
}
