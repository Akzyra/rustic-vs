use crate::core::install::Install;
use crate::ui::components::*;
use crate::ui::installs::AddInstallState::{Done, Unpacking};
use crate::vs::{Progress, unpack_vs};
use freya::i18n::t;
use freya::prelude::*;
use log::{error, info};
use std::ops::Deref;
use std::path::PathBuf;
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

#[derive(Debug, Clone, PartialEq)]
enum AddInstallState {
    New,
    Downloading(f32), // TODO: more data?
    Unpacking(Progress),
    Done,
}

impl Component for AddInstallPopup {
    fn render(&self) -> impl IntoElement {
        let popup_scope_id = current_scope_id();

        let mut open = self.show;
        let mut id = use_state(String::new);
        let version = use_state(|| "fake download".to_string());
        let mut installs = use_consume::<State<Vec<Install>>>();
        let mut install = use_state(|| None::<Install>);

        //TODO: read global cache of game version using API -> select box

        let mut state = use_state(|| AddInstallState::New);
        let download_percent = use_memo(move || match state.read().deref() {
            AddInstallState::New => -1f32,
            AddInstallState::Downloading(percent) => *percent,
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
            AddInstallState::Downloading(percent) => format!("downloading {:.1}%", percent),
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
            open.set(false);
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
            open.set(false);
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

                // set game version to None, will be loaded from disk when done
                let new_install = Install::create(&cwd, new_id, None);
                if new_install.id != "test"
                    && installs.peek().iter().any(|i| i.id == new_install.id)
                {
                    log::warn!(
                        "install `{}` already exists, not adding a duplicate",
                        new_install.id
                    );
                    return;
                }

                let out_path = new_install.path.clone();
                install.set(Some(new_install));

                // move unpack to blocking pool
                let (tx, mut rx) = watch::channel(AddInstallState::New);
                let job = tokio::task::spawn_blocking(move || {
                    // download file
                    // TODO: call download
                    info!("fake download {}", game_version);
                    for n in 1..101 {
                        std::thread::sleep(std::time::Duration::from_millis(20));
                        tx.send_replace(AddInstallState::Downloading(n as f32));
                    }

                    // unpack archive
                    //TODO: use downloaded file
                    let exe = PathBuf::from("D:/Vintage Story/test/vs_install_win-x64_1.22.7.exe");
                    let tar =
                        PathBuf::from("D:/Vintage Story/test/vs_client_linux-x64_1.22.7.tar.gz");

                    let res = unpack_vs(&tar, &out_path, |progress| {
                        tx.send_replace(Unpacking(progress));
                    });
                    if let Err(e) = res {
                        log::error!("unpack failed: {:?}", e);
                    };

                    tx.send_replace(Done);
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
                                error!("{}", e);
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
            .maybe(open(), |popup| {
                popup.child(PopupTitle::new(t!("add_install"))).child(
                    PopupContent::new()
                        .child(
                            flex_rect(Direction::Vertical, SPACING_MD)
                                .child(form_row(
                                    &label_width,
                                    t!("install_id"),
                                    Input::new(id)
                                        .width(Size::flex(1.0))
                                        .auto_focus(true)
                                        .enabled(matches!(*state.read(), AddInstallState::New))
                                        .placeholder(t!("install_id")),
                                ))
                                .child(form_row(
                                    &label_width,
                                    t!("install_version"),
                                    // TOOD: add version select
                                    label().text(version.read().clone()),
                                ))
                                .maybe_child(download_percent.read().is_sign_positive().then(
                                    || {
                                        form_row(
                                            &label_width,
                                            t!("download"),
                                            ProgressBar::new(*download_percent.read())
                                                .width(Size::flex(1.0)),
                                        )
                                    },
                                ))
                                .maybe_child(unpacking_percent.read().is_sign_positive().then(
                                    || {
                                        form_row(
                                            &label_width,
                                            t!("unpacking"),
                                            ProgressBar::new(*unpacking_percent.read())
                                                .width(Size::flex(1.0)),
                                        )
                                    },
                                ))
                                .child(
                                    rect().overflow(Overflow::Clip).width(Size::fill()).child(
                                        label().max_lines(1).text(status_text.read().clone()),
                                    ),
                                )
                                .child(
                                    label()
                                        .overline()
                                        .text("Mockup, does not download new versions yet!"),
                                ),
                        )
                        .child(match *state.read() {
                            AddInstallState::New => PopupButtons::new()
                                .child(Button::new().on_press(on_close).child(t!("cancel")))
                                .child(
                                    Button::new()
                                        .filled()
                                        .on_press(move |_| {
                                            let id = id.peek().clone();
                                            let version = version.peek().clone();
                                            on_download.call((id, version));
                                        })
                                        .child(t!("download")),
                                ),
                            AddInstallState::Downloading(_) => PopupButtons::new().child(
                                Button::new().enabled(false).child("TODO: implement cancel"),
                            ),
                            Unpacking(_) => PopupButtons::new().child(
                                Button::new().enabled(false).child("TODO: implement cancel"),
                            ),
                            Done => PopupButtons::new()
                                .child(Button::new().on_press(on_ok).child(t!("ok"))),
                        }),
                )
            })
    }
}
