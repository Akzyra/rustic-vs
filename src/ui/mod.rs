mod components;

use std::hash::Hash;

use crate::core::install::{Install, load_installs};
use crate::core::instance::{Instance, load_instances};
use crate::ui::components::*;
use freya::i18n::*;
use freya::icons;
use freya::prelude::*;
use freya::router::*;

pub fn app() -> impl IntoElement {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US")).with_locale(Locale::new_static(
            langid!("en-US"),
            include_str!("../../i18n/en-US.ftl"),
        ))
    });

    use_init_theme(light_theme); // TODO: read/write settings file

    let cwd = std::env::current_dir().expect("CWD must exist");
    use_provide_context(|| State::create(load_installs(&cwd)));
    use_provide_context(|| State::create(load_instances(&cwd)));

    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::News))
}

#[derive(Routable, Clone, PartialEq, Hash)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    News,
    #[route("/versions")]
    Installs,
    #[route("/instances")]
    Instances,
    #[route("/settings")]
    Settings,
    #[route("/edit/:instance_id")]
    InstanceEdit { instance_id: String },
}

#[derive(PartialEq)]
struct Layout;

impl Component for Layout {
    fn render(&self) -> impl IntoElement {
        let instances = use_consume::<State<Vec<Instance>>>();
        let mut selected_instance_idx = use_state(|| 0);

        rect()
            .theme_color()
            .theme_background()
            .native_router()
            .content(Content::Flex)
            .horizontal()
            .child(
                rect()
                    .content(Content::Flex)
                    .spacing(SPACING_SM)
                    .padding(SPACING_SM)
                    .vertical()
                    .width(Size::px(240.0))
                    .height(Size::fill())
                    .background(get_theme_or_default().read().colors.surface_tertiary)
                    .child(sidebar_link(
                        Route::News,
                        icons::lucide::rss(),
                        t!("menu_news"),
                        t!("menu_news_tt"),
                    ))
                    .child(sidebar_link(
                        Route::Installs,
                        icons::lucide::boxes(),
                        t!("menu_installs"),
                        t!("menu_installs_tt"),
                    ))
                    .child(sidebar_link(
                        Route::Instances,
                        icons::lucide::folder_cog(),
                        t!("menu_instances"),
                        t!("menu_instances_tt"),
                    ))
                    .child(sidebar_link(
                        Route::Settings,
                        icons::lucide::settings(),
                        t!("menu_settings"),
                        t!("menu_settings_tt"),
                    ))
                    .child(rect().height(Size::flex(1.)))
                    .child(
                        Select::new()
                            .width(Size::flex(1.0))
                            // TODO: custom element for instance in menu and selected
                            .selected_item(instances.read()[selected_instance_idx()].name.clone())
                            .children(instances.read().iter().enumerate().map(|(i, inst)| {
                                MenuItem::new()
                                    .selected(selected_instance_idx() == i)
                                    .on_press(move |_| {
                                        selected_instance_idx.set(i);

                                        // update edit view if active
                                        let rc = RouterContext::get();
                                        if let Route::InstanceEdit { instance_id: _ } = rc.current()
                                        {
                                            let _ = rc.push(Route::InstanceEdit {
                                                instance_id: instances.read()
                                                    [selected_instance_idx()]
                                                .id
                                                .clone(),
                                            });
                                        };
                                    })
                                    .child(inst.name.clone())
                                    .into()
                            })),
                    )
                    .child(sidebar_link(
                        Route::InstanceEdit {
                            instance_id: instances.read()[selected_instance_idx()].id.clone(),
                        },
                        icons::lucide::bolt(),
                        t!("menu_instance_edit"),
                        t!("menu_instance_edit_tt"),
                    ))
                    .child(
                        Button::new()
                            .height(Size::px(56.0))
                            .width(Size::Fill)
                            .filled()
                            .child(t!(
                                "play",
                                name: instances.read()[selected_instance_idx()].name.clone()
                            )),
                    ),
            )
            .child(
                rect()
                    .theme_background()
                    .theme_color()
                    .content(Content::Flex)
                    .vertical()
                    .padding(SPACING_SM)
                    .spacing(SPACING_SM)
                    .width(Size::fill())
                    .height(Size::fill())
                    .child(Outlet::<Route>::new()),
            )
    }
}

#[derive(PartialEq)]
struct News;

impl Component for News {
    fn render(&self) -> impl IntoElement {
        main_rect(t!("news_header")).child("TODO: add this..")
    }
}

#[derive(PartialEq)]
struct Installs;

impl Component for Installs {
    fn render(&self) -> impl IntoElement {
        let installs = use_consume::<State<Vec<Install>>>();

        main_rect(t!("installs_header")).child(
            Table::new()
                .column_widths([Size::px(100.0), Size::px(100.0), Size::px(100.0)])
                .child(
                    TableHead::new().child(
                        TableRow::new()
                            .child(TableCell::new().child(t!("installs_column_id")))
                            .child(TableCell::new().child(t!("installs_column_version")))
                            .child(TableCell::new().child(t!("installs_column_actions"))),
                    ),
                )
                .child(TableBody::new().child(ScrollView::new().children(
                    installs.read().iter().enumerate().map(|(i, install)| {
                        TableRow::new()
                            .key(i)
                            .child(TableCell::new().child(install.id.clone()))
                            .child(
                                TableCell::new().child(
                                    install
                                        .game_version
                                        .clone()
                                        .unwrap_or_else(|| "<unk>".to_string()),
                                ),
                            )
                            .child(TableCell::new().child("todo"))
                            .into()
                    }),
                ))),
        )
    }
}

#[derive(PartialEq)]
struct Instances;

impl Component for Instances {
    fn render(&self) -> impl IntoElement {
        let instances = use_consume::<State<Vec<Instance>>>();

        main_rect(t!("instances_header")).child(
            Table::new()
                .column_widths([
                    Size::px(100.0),
                    Size::flex(1.0),
                    Size::px(100.0),
                    Size::px(100.0),
                ])
                .child(
                    TableHead::new().child(
                        TableRow::new()
                            .child(TableCell::new().child(t!("instances_column_id")))
                            .child(TableCell::new().child(t!("instances_column_name")))
                            .child(TableCell::new().child(t!("instances_column_install")))
                            .child(TableCell::new().child(t!("instances_column_actions"))),
                    ),
                )
                .child(TableBody::new().child(ScrollView::new().children(
                    instances.read().iter().enumerate().map(|(i, instance)| {
                        TableRow::new()
                            .key(i)
                            .child(TableCell::new().child(instance.id.clone()))
                            .child(TableCell::new().child(instance.name.clone()))
                            .child(
                                TableCell::new().child(
                                    instance
                                        .install_id
                                        .clone()
                                        .unwrap_or_else(|| "<unk>".to_string()),
                                ),
                            )
                            .child(TableCell::new().child("todo"))
                            .into()
                    }),
                ))),
        )
    }
}

#[derive(PartialEq)]
struct Settings;

impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        let mut theme = use_theme();
        let is_light = theme.read().name == "light";

        main_rect(t!("settings_header")).child(rect().child(t!("settings_theme")).child(
            Switch::new().toggled(is_light).on_toggle(move |_| {
                theme.set(if is_light {
                    dark_theme()
                } else {
                    light_theme()
                });
            }),
        ))
    }
}

#[derive(PartialEq)]
struct InstanceEdit {
    instance_id: String,
}

impl Component for InstanceEdit {
    fn render(&self) -> impl IntoElement {
        let instance_id = self.instance_id.clone();
        let instances = use_consume::<State<Vec<Instance>>>().peek();
        let instance = instances.iter().find(|i| i.id == instance_id).unwrap();

        main_rect(t!("instance_edit_header", name: instance.name.clone())).child("todo: add this..")
    }
}
