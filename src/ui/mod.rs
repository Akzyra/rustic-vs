use std::hash::Hash;

use crate::core::install::{Install, load_installs};
use crate::core::instance::{Instance, load_instances};
use freya::i18n::*;
use freya::icons;
use freya::prelude::*;
use freya::router::*;

pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 16.0;

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

fn sidebar_item(target: Route, icon: Bytes, title: String) -> impl IntoElement {
    ActivableRoute::new(
        target.clone(),
        Link::new(target.clone()).child(
            SideBarItem::new().padding(SPACING_SM).child(
                rect()
                    .content(Content::Flex)
                    .horizontal()
                    .spacing(SPACING_SM)
                    .padding(0.0)
                    .cross_align(Alignment::Center)
                    .main_align(Alignment::Center)
                    .child(
                        SvgViewer::new((target, icon))
                            .width(Size::px(24.0))
                            .height(Size::px(24.0)),
                    )
                    .child(
                        label()
                            .text(title)
                            .font_weight(FontWeight::BOLD)
                            .font_size(18.0),
                    ),
            ),
        ),
    )
    .exact(true)
}

#[derive(Routable, Clone, PartialEq, Hash)]
#[rustfmt::skip]
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
        let mut selected_instance = use_state(|| 0);

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
                    .child(sidebar_item(
                        Route::News,
                        icons::lucide::rss(),
                        t!("menu_news"),
                    ))
                    .child(sidebar_item(
                        Route::Installs,
                        icons::lucide::boxes(),
                        t!("menu_installs"),
                    ))
                    .child(sidebar_item(
                        Route::Instances,
                        icons::lucide::folder_cog(),
                        t!("menu_instances"),
                    ))
                    .child(sidebar_item(
                        Route::Settings,
                        icons::lucide::settings(),
                        t!("menu_settings"),
                    ))
                    .child(rect().height(Size::flex(1.)))
                    .child(
                        Select::new()
                            .width(Size::flex(1.0))
                            // TODO: custom element for instance in menu and selected
                            .selected_item(instances.read()[selected_instance()].name.clone())
                            .children(instances.read().iter().enumerate().map(|(i, inst)| {
                                MenuItem::new()
                                    .selected(selected_instance() == i)
                                    .on_press(move |_| {
                                        selected_instance.set(i);

                                        // update edit view if active
                                        let rc = RouterContext::get();
                                        if let Route::InstanceEdit { instance_id: _ } = rc.current()
                                        {
                                            let _ = rc.push(Route::InstanceEdit {
                                                instance_id: instances.read()[selected_instance()]
                                                    .id
                                                    .clone(),
                                            });
                                        };
                                    })
                                    .child(inst.name.clone())
                                    .child("some other text")
                                    .into()
                            })),
                    )
                    .child(sidebar_item(
                        Route::InstanceEdit {
                            instance_id: instances.read()[selected_instance()].id.clone(),
                        },
                        icons::lucide::bolt(),
                        t!("menu_management"),
                    ))
                    .child(
                        Button::new()
                            .height(Size::px(56.0))
                            .width(Size::Fill)
                            .filled()
                            .child(t!("play", name: instances.read()[selected_instance()].name.clone())),
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
        rect().child("TODO: RSS feed")
    }
}

#[derive(PartialEq)]
struct Installs;

impl Component for Installs {
    fn render(&self) -> impl IntoElement {
        let installs = use_consume::<State<Vec<Install>>>();

        rect()
            .theme_background()
            .content(Content::Flex)
            .vertical()
            .padding(SPACING_SM)
            .spacing(SPACING_MD)
            .child(
                label()
                    .font_weight(FontWeight::BOLD)
                    .font_size(20.0)
                    .text(t!("installs_header")),
            )
            .child(
                Table::new()
                    .column_widths([Size::flex(1.0), Size::flex(1.0), Size::flex(1.0)])
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

        rect()
            .theme_background()
            .content(Content::Flex)
            .vertical()
            .padding(SPACING_SM)
            .spacing(SPACING_MD)
            .child(
                label()
                    .font_weight(FontWeight::BOLD)
                    .font_size(20.0)
                    .text(t!("instances_header")),
            )
            .child(
                Table::new()
                    .column_widths([
                        Size::flex(2.),
                        Size::flex(2.0),
                        Size::flex(1.0),
                        Size::flex(1.0),
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

        rect()
            .theme_background()
            .content(Content::Flex)
            .vertical()
            .padding(SPACING_SM)
            .spacing(SPACING_MD)
            .child(
                label()
                    .font_weight(FontWeight::BOLD)
                    .font_size(20.0)
                    .text(t!("settings_header")),
            )
            .child(rect().child(t!("settings_theme")).child(
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

        rect()
            .child("")
            .child("TODO: Management for selected instance")
            .child(instance_id)
    }
}
