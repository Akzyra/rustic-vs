mod components;

use std::hash::Hash;

use crate::core::install::{Install, load_installs};
use crate::core::instance::{Instance, load_instances};
use crate::core::rss::load_news_feed;
use crate::core::settings::Settings as SettingsStruct;
use crate::core::settings::THEMES;
use crate::core::settings::Theme;
use crate::core::settings::load_settings;
use crate::ui::components::*;
use freya::i18n::*;
use freya::icons;
use freya::prelude::*;
use freya::router::*;
use rss::Channel;

fn get_theme(theme: &Theme) -> freya::prelude::Theme {
    match theme {
        Theme::Light => light_theme(),
        Theme::Dark => dark_theme(),
    }
}

pub fn app() -> impl IntoElement {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US")).with_locale(Locale::new_static(
            langid!("en-US"),
            include_str!("../../i18n/en-US.ftl"),
        ))
    });
    let cwd = std::env::current_dir().expect("CWD must exist");
    let settings = load_settings(&cwd);

    use_init_theme(|| get_theme(&settings.theme));

    use_provide_context(|| State::create(settings));
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
        // TODO: reload everytime route to this, cache it?
        let future = use_future(move || async move {
            if let Ok(channel) = load_news_feed().await {
                channel
            } else {
                Channel::default()
            }
        });

        main_rect(t!("news_header")).child(match &*future.state() {
            FutureState::Pending => label().text("Pending...").into_element(),
            FutureState::Loading => label().text("Loading...").into_element(),
            FutureState::Fulfilled(channel) => ScrollView::new()
                .width(Size::Fill)
                .spacing(SPACING_MD)
                .children(channel.items.clone().into_iter().flat_map(|item| {
                    let Some(title) = item.title else {
                        return None;
                    };
                    let Some(link) = item.link else {
                        return None;
                    };
                    let Some(date) = item.pub_date else {
                        return None;
                    };

                    let short_date = chrono::DateTime::parse_from_rfc2822(&*date)
                        .map(|dt| format!("{}", dt.format("%Y-%m-%d")))
                        .unwrap_or(date);

                    Some(rss_row(title, short_date, link).into_element())
                }))
                .into_element(),
        })
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
        let mut settings = use_consume::<State<SettingsStruct>>();
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
