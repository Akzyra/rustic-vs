use crate::core::install::load_installs;
use crate::core::instance::{Instance, load_instances};
use crate::core::settings::load_settings;
use crate::ui::components::*;
use crate::ui::get_theme;
use crate::vs::api::{Downloads, FeedEntry, load_versions};
use freya::i18n::*;
use freya::icons::lucide;
use freya::prelude::*;
use freya::router::*;

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
    use_provide_context(|| State::create(Vec::<FeedEntry>::new()));

    let mut version_downloads = use_provide_context(|| State::create(Downloads::default()));
    use_future(move || async move {
        match load_versions().await {
            Ok(dl) => {
                version_downloads.set(dl);
            }
            Err(e) => {
                log::error!("Failed to load versions: {}", e);
            }
        }
    });

    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::News))
}

#[derive(Routable, Clone, PartialEq, Hash)]
pub(super) enum Route {
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

// implementations for Route enum
use crate::ui::installs::Installs;
use crate::ui::instance_edit::InstanceEdit;
use crate::ui::instances::Instances;
use crate::ui::news::News;
use crate::ui::settings::Settings;

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
                        lucide::rss(),
                        tid!("menu_news"),
                        tid!("menu_news_tt"),
                    ))
                    .child(sidebar_link(
                        Route::Installs,
                        lucide::boxes(),
                        tid!("menu_installs"),
                        tid!("menu_installs_tt"),
                    ))
                    .child(sidebar_link(
                        Route::Instances,
                        lucide::folder_cog(),
                        tid!("menu_instances"),
                        tid!("menu_instances_tt"),
                    ))
                    .child(sidebar_link(
                        Route::Settings,
                        lucide::settings(),
                        tid!("menu_settings"),
                        tid!("menu_settings_tt"),
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
                            })),
                    )
                    .child(sidebar_link(
                        Route::InstanceEdit {
                            instance_id: instances.read()[selected_instance_idx()].id.clone(),
                        },
                        lucide::bolt(),
                        tid!("menu_instance_edit"),
                        tid!("menu_instance_edit_tt"),
                    ))
                    .child(
                        Button::new()
                            .height(Size::px(56.0))
                            .width(Size::Fill)
                            .filled()
                            .child(tid!(
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
