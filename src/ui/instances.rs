use crate::core::install::Install;
use crate::core::instance::Instance;
use crate::ui::components::*;
use freya::i18n::tid;
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
                            .text(tid!("instances_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(
                        Button::new()
                            .on_press(move |_| show_add.set(true))
                            .child(tid!("add_instance")),
                    ),
            )
            .child(
                Table::new()
                    .column_widths([
                        Size::flex(1.3),
                        Size::flex(1.0),
                        Size::px(110.0),
                        Size::px(90.0),
                    ])
                    .child(
                        TableRow::new()
                            .child(
                                cell_fix(Alignment::Center).child(
                                    label()
                                        .font_weight(FontWeight::BOLD)
                                        .text(tid!("instance_name")),
                                ),
                            )
                            .child(
                                cell_fix(Alignment::Center).child(
                                    label()
                                        .font_weight(FontWeight::BOLD)
                                        .text(tid!("install_id")),
                                ),
                            )
                            .child(
                                cell_fix(Alignment::Center).child(
                                    label().font_weight(FontWeight::BOLD).text(tid!("size")),
                                ),
                            )
                            .child(cell_fix(Alignment::Center).child(
                                label().font_weight(FontWeight::BOLD).text(tid!("actions")),
                            )),
                    )
                    .child(
                        ScrollView::new().children(instances.read().iter().enumerate().map(
                            |(i, instance)| {
                                let instance_path = instance.path.clone();
                                TableRow::new()
                                    .key(i)
                                    .child(cell_fix(Alignment::Center).child(instance.name.clone()))
                                    .child(
                                        cell_fix(Alignment::Center).child(
                                            instance
                                                .install_id
                                                .clone()
                                                .unwrap_or_else(|| "—".to_string()),
                                        ),
                                    )
                                    .child(cell_fix(Alignment::Center).child(
                                        instance.size.clone().unwrap_or_else(|| "—".to_string()),
                                    ))
                                    .child(
                                        cell_fix(Alignment::Center)
                                            .spacing(SPACING_XS)
                                            .child(
                                                icon_button(Size::px(16.0), lucide::folder_open())
                                                    .flat()
                                                    .on_press(move |_| {
                                                        if let Err(e) = open::that(&instance_path) {
                                                            log::error!("failed open path: {}", e)
                                                        };
                                                    }),
                                            )
                                            .child("todo"),
                                    )
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

                let Ok(cwd) = std::env::current_dir() else {
                    log::error!("failed to get current dir, cannot save instance");
                    return;
                };

                let mut instance = Instance::create(&cwd, new_name, install_id);
                if instances.peek().iter().any(|i| i.id == instance.id) {
                    log::warn!(
                        "instance `{}` already exists, not adding a duplicate",
                        instance.id
                    );
                    return;
                }

                if let Err(e) = instance.save() {
                    log::error!("failed to save instance `{}`: {}", instance.id, e);
                    return;
                }

                instance.reload();
                instances.write().push(instance);

                // reset
                open.set(false);
                name.set(String::new());
                selected_install_idx.set(None);
            })
            .into();

        let label_width = Size::px(100.0);
        Popup::new()
            .width(Size::px(450.0))
            .on_close_request(on_close.clone())
            .maybe(open(), |popup| {
                popup
                    .child(PopupTitle::new(tid!("add_instance_title")))
                    .child(
                        PopupContent::new().child(
                            flex_rect(Direction::Vertical, SPACING_MD)
                                .child(form_row(
                                    &label_width,
                                    tid!("instance_name"),
                                    Input::new(name)
                                        .width(Size::flex(1.0))
                                        .auto_focus(true)
                                        .placeholder(tid!("instance_name")),
                                ))
                                .child(form_row(
                                    &label_width,
                                    tid!("install_id"),
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
                                            icon_button(Size::px(24.0), lucide::x())
                                                .on_press(move |_| selected_install_idx.set(None)),
                                        ),
                                )),
                        ),
                    )
                    .child(
                        PopupButtons::new()
                            .child(
                                Button::new()
                                    .on_press(move |_| on_close.call(()))
                                    .child(tid!("cancel")),
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
                                    .child(tid!("create")),
                            ),
                    )
            })
    }
}
