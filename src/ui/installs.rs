use crate::core::install::Install;
use crate::ui::components::*;
use crate::ui::installs::AddInstallState::{Done, Unpacking};
use crate::vs::api::{DownloadPlatform, Downloads};
use crate::vs::{Progress, download_vs, unpack_vs};
use freya::i18n::t;
use freya::prelude::*;
use std::ops::Deref;
use tokio::sync::watch;

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
                            .text(tid!("installs_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(
                        Button::new()
                            .on_press(move |_| show_add.set(true))
                            .child(tid!("add_install")),
                    ),
            )
            .child(
                Table::new()
                    .column_widths([Size::flex(1.0), Size::flex(1.0), Size::px(100.0)])
                    .child(
                        TableRow::new()
                            .child(cell_fix(Alignment::Center).child(
                                label().font_weight(FontWeight::BOLD).text(t!("install_id")),
                            ))
                            .child(
                                cell_fix(Alignment::Center).child(
                                    label()
                                        .font_weight(FontWeight::BOLD)
                                        .text(t!("install_version")),
                                ),
                            )
                            .child(
                                cell_fix(Alignment::Center).child(
                                    label().font_weight(FontWeight::BOLD).text(t!("actions")),
                                ),
                            ),
                    )
                    .child(
                        ScrollView::new().children(installs.read().iter().enumerate().map(
                            |(i, install)| {
                                TableRow::new()
                                    .key(i)
                                    .child(cell_fix(Alignment::Center).child(install.id.clone()))
                                    .child(
                                        cell_fix(Alignment::Center).child(
                                            install
                                                .game_version
                                                .clone()
                                                .unwrap_or_else(|| "—".to_string()),
                                        ),
                                    )
                                    .child(cell_fix(Alignment::Center).child("todo"))
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

#[derive(Debug, Clone, PartialEq)]
enum AddInstallState {
    New,
    Downloading(Progress),
    Unpacking(Progress),
    Done,
}

impl Component for AddInstallPopup {
    fn render(&self) -> impl IntoElement {
        let popup_scope_id = current_scope_id();

        let mut show = self.show;
        let mut id = use_state(String::new);
        let unstable = use_state(|| false);
        let selected_version = use_state(|| "1.22.7".to_string());

        let downloads = use_consume::<State<Downloads>>();
        let mut installs = use_consume::<State<Vec<Install>>>();
        let mut install = use_state(|| None::<Install>);

        //TODO: implement select box
        // TODO: remove hardcoded values
        let download_version = use_memo(move || {
            let version = selected_version.read().clone();
            let platform = DownloadPlatform::Windows;

            let dls = downloads.read();
            let map = if unstable() {
                &dls.unstable
            } else {
                &dls.stable
            };

            map.get(&version)
                .expect("version must exist")
                .get(&platform)
                .expect("platform must exist")
                .clone()
        });

        let mut state = use_state(|| AddInstallState::New);
        let download_percent = use_memo(move || match state.read().deref() {
            AddInstallState::New => -1f32,
            AddInstallState::Downloading(progress) => (progress.bytes_written * 100)
                .checked_div(progress.bytes_total)
                .unwrap_or(0) as f32,
            Unpacking(_) => 100f32,
            Done => 100f32,
        });
        let unpacking_percent = use_memo(move || match state.read().deref() {
            AddInstallState::New => -1f32,
            AddInstallState::Downloading(_) => 0f32,
            Unpacking(progress) => (progress.bytes_written * 100)
                .checked_div(progress.bytes_total)
                .unwrap_or(0) as f32,
            Done => 100f32,
        });
        let status_text = use_memo(move || match state.read().deref() {
            AddInstallState::New => String::new(),
            AddInstallState::Downloading(progress) => {
                let total = humansize::format_size(progress.bytes_total, humansize::BINARY);
                let written = humansize::format_size(progress.bytes_written, humansize::BINARY);

                format!("{}/{} \u{2022} {}", written, total, progress.current_file)
            }
            Unpacking(progress) => {
                let total = humansize::format_size(progress.bytes_total, humansize::BINARY);
                let written = humansize::format_size(progress.bytes_written, humansize::BINARY);
                let filename = match progress.current_file.rsplit_once('/') {
                    None => &progress.current_file,
                    Some((_, filename)) => filename,
                };

                format!("{}/{} \u{2022} {}", written, total, filename)
            }
            Done => "DONE".to_string(),
        });

        let on_close: EventHandler<_> = (move |_| {
            show.set(false);
            id.set(String::new());
            //TODO: reset select
            state.set(AddInstallState::New);
        })
        .into();

        let on_ok: EventHandler<_> = (move |_| {
            // reload to get game version
            //TODO: fix use of partially moved value: `install`
            let new_install = install.read().clone();
            let mut new_install2 = new_install.unwrap();
            new_install2.reload();
            installs.write().push(new_install2.clone());

            // reset
            show.set(false);
            id.set(String::new());
            //TODO: reset select
            state.set(AddInstallState::New);
        })
        .into();

        let on_download: EventHandler<(String, String)> =
            (move |(raw_id, game_version): (String, String)| {
                //TODO: display error in UI, show if name cannot be used -> input on_validation
                let new_id = raw_id.trim().to_string();
                if new_id.is_empty() {
                    return;
                }

                let Ok(cwd) = std::env::current_dir() else {
                    log::error!("failed to get current dir, cannot save install");
                    return;
                };

                let new_install = Install::create(&cwd, new_id, Some(game_version));
                if new_install.id != "test"
                    && installs.peek().iter().any(|i| i.id == new_install.id)
                {
                    // TODO: show error in UI
                    log::warn!(
                        "install `{}` already exists, not adding a duplicate",
                        new_install.id
                    );
                    return;
                }

                let dl_info = download_version.peek().clone();
                let out_path = new_install.path.clone();
                install.set(Some(new_install));

                // move unpack to blocking pool
                let (tx, mut rx) = watch::channel(AddInstallState::New);
                let job = tokio::task::spawn_blocking(move || {
                    // download file
                    let res = download_vs(&dl_info, |progress| {
                        tx.send_replace(AddInstallState::Downloading(progress));
                    });
                    let archive_path = match res {
                        Ok(archive_path) => archive_path,
                        Err(e) => {
                            // TODO: show error in UI
                            log::error!("download failed: {:?}", e);
                            return;
                        }
                    };

                    // unpack archive
                    let res = unpack_vs(&archive_path, &out_path, |progress| {
                        tx.send_replace(Unpacking(progress));
                    });
                    if let Err(e) = res {
                        // TODO: show error in UI
                        log::error!("unpack failed: {:?}", e);
                        return;
                    };

                    tx.send_replace(Done);
                });

                // handle messages
                spawn_in_scope(
                    async move {
                        // do-while pattern
                        loop {
                            let new_state = rx.borrow_and_update().deref().clone();
                            state.set(new_state);
                            if rx.changed().await.is_err() {
                                break;
                            }
                        }

                        // get final message
                        let new_state = rx.borrow();
                        state.set(new_state.clone());

                        match job.await {
                            Ok(()) => {}
                            Err(e) => {
                                //TODO: show in UI ?
                                log::error!("{}", e);
                            }
                        }
                    },
                    popup_scope_id,
                );
            })
            .into();

        let label_width = Size::px(100.0);
        Popup::new()
            .width(Size::px(450.0))
            //.on_close_request(on_close.clone())
            .maybe(show(), |popup| {
                popup.child(PopupTitle::new(tid!("add_install"))).child(
                    PopupContent::new()
                        .child(
                            flex_rect(Direction::Vertical, SPACING_MD)
                                .child(form_row(
                                    &label_width,
                                    tid!("install_id"),
                                    Input::new(id)
                                        .width(Size::flex(1.0))
                                        .auto_focus(true)
                                        .enabled(matches!(*state.read(), AddInstallState::New))
                                        .placeholder(tid!("install_id")),
                                ))
                                .child(form_row(
                                    &label_width,
                                    tid!("install_version"),
                                    // TOOD: add version select
                                    label().text(selected_version.read().clone()),
                                ))
                                .maybe_child(download_percent.read().is_sign_positive().then(
                                    || {
                                        form_row(
                                            &label_width,
                                            tid!("download"),
                                            ProgressBar::new(*download_percent.read())
                                                .width(Size::flex(1.0)),
                                        )
                                    },
                                ))
                                .maybe_child(unpacking_percent.read().is_sign_positive().then(
                                    || {
                                        form_row(
                                            &label_width,
                                            tid!("unpacking"),
                                            ProgressBar::new(*unpacking_percent.read())
                                                .width(Size::flex(1.0)),
                                        )
                                    },
                                ))
                                .child(
                                    rect().overflow(Overflow::Clip).width(Size::fill()).child(
                                        label().max_lines(1).text(status_text.read().clone()),
                                    ),
                                ),
                        )
                        .child(match *state.read() {
                            AddInstallState::New => PopupButtons::new()
                                .child(Button::new().on_press(on_close).child(tid!("cancel")))
                                .child(
                                    Button::new()
                                        .filled()
                                        .on_press(move |_| {
                                            let id = id.peek().clone();
                                            let version = selected_version.peek().clone();
                                            on_download.call((id, version));
                                        })
                                        .child(tid!("download")),
                                ),
                            AddInstallState::Downloading(_) => PopupButtons::new().child(
                                Button::new()
                                    .enabled(false)
                                    .flat()
                                    .child("TODO: implement cancel"),
                            ),
                            Unpacking(_) => PopupButtons::new().child(
                                Button::new()
                                    .enabled(false)
                                    .flat()
                                    .child("TODO: implement cancel"),
                            ),
                            Done => PopupButtons::new()
                                .child(Button::new().on_press(on_ok).child(tid!("ok"))),
                        }),
                )
            })
    }
}
