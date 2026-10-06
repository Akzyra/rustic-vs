use crate::ui::components::*;
use crate::vs::api::{FeedEntry, load_news_feed};
use freya::html::{HtmlHandle, HtmlSource, HtmlViewer};
use freya::i18n::t;
use freya::icons::lucide;
use freya::prelude::*;
use std::error::Error;
use std::ops::Not;
use std::time::Duration;

#[derive(PartialEq)]
pub struct News;

impl Component for News {
    fn render(&self) -> impl IntoElement {
        let mut feed = use_consume::<State<Option<Vec<FeedEntry>>>>();
        let mut future = use_future(move || async move {
            if feed.peek().is_none() {
                match load_news_feed() {
                    Ok(entries) => {
                        feed.set(Some(entries));
                    }
                    Err(e) => {
                        log::error!("failed to load news feed: {}", e);
                    }
                }
            }
        });
        let loading = feed.read().is_none();
        flex_rect(Direction::Vertical, SPACING_MD)
            .theme_background()
            .padding(SPACING_SM)
            .child(
                flex_rect(Direction::Horizontal, SPACING_SM)
                    .child(
                        label()
                            .font_weight(FontWeight::BOLD)
                            .font_size(20.0)
                            .text(t!("news_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(
                        Button::new()
                            .padding(6.0)
                            .on_press(move |_| {
                                feed.set(None);
                                future.start();
                            })
                            .child(
                                SvgViewer::new(lucide::rotate_cw())
                                    .width(Size::px(24.0))
                                    .height(Size::px(24.0)),
                            ),
                    ),
            )
            .child(
                Skeleton::new(loading)
                    .expanded()
                    .animation(SkeletonAnimation::Shimmer)
                    .duration(Duration::from_secs(2))
                    // needs maybe_child since expect() would fail on None
                    .maybe_child(loading.not().then(|| {
                        ScrollView::new()
                            .expanded()
                            .spacing(SPACING_MD)
                            .scroll_with_arrows(false)
                            .drag_scrolling(false)
                            .children(
                                feed.read()
                                    .as_ref()
                                    .expect("maybe_child() guard")
                                    .iter()
                                    .map(|feed_entry: &FeedEntry| RssRow {
                                        feed_entry: feed_entry.clone(),
                                    }),
                            )
                    })),
            )
    }
}
#[derive(PartialEq)]
struct RssRow {
    feed_entry: FeedEntry,
}

impl Component for RssRow {
    fn render(&self) -> impl IntoElement {
        let mut open = use_state(|| false);
        let short_date = format!("{}", self.feed_entry.date.format("%Y-%m-%d"));

        // since the HtmlViewer does not resize dynamically we force a known size
        // then handle overflow/scrolling inside HTML
        // TODO: dark mode?
        let fixed_size = 400.0;
        let fixed_html = format!(
            r#"
            <body style="box-sizing: border-box; height:{}px; margin: 0px; padding: 0px;
              overflow-y: scroll; scrollbar-width: thin;
            ">
            <div style="padding: 0px 15px; background: #f6e8c9; border: 1px solid #000; border-radius: 10px;">
              {}
            </div>
            </body>
            "#,
            fixed_size - 2.0, // borders...
            self.feed_entry.description
        );

        Card::new()
            .outline()
            .width(Size::fill())
            .padding(SPACING_SM)
            .height(Size::auto())
            .child(
                flex_rect(Direction::Vertical, 0.0)
                    .width(Size::fill())
                    .height(Size::auto())
                    .child(
                        // header
                        flex_rect(Direction::Horizontal, SPACING_MD)
                            .width(Size::fill())
                            .height(Size::auto())
                            .cross_align(Alignment::Center)
                            .child(
                                Button::new().flat().width(Size::flex(1.0)).child(
                                    flex_rect(Direction::Horizontal, SPACING_SM)
                                        .width(Size::fill())
                                        .on_press(move |_| open.toggle())
                                        .child(
                                            SvgViewer::new(if open() {
                                                lucide::chevron_down()
                                            } else {
                                                lucide::chevron_up()
                                            })
                                            .width(Size::px(24.0))
                                            .height(Size::px(24.0)),
                                        )
                                        .child(
                                            label()
                                                .text(self.feed_entry.title.clone())
                                                .font_weight(FontWeight::BOLD)
                                                .width(Size::flex(1.0)),
                                        ),
                                ),
                            )
                            .child(
                                Link::new(self.feed_entry.link.clone()).child(
                                    Button::new().flat().width(Size::auto()).child(
                                        flex_rect(Direction::Horizontal, SPACING_SM)
                                            .child(short_date)
                                            .child(
                                                SvgViewer::new(lucide::external_link())
                                                    .width(Size::px(20.0))
                                                    .height(Size::px(20.0)),
                                            ),
                                    ),
                                ),
                            ),
                        // header end
                    )
                    .maybe_child(open.read().then(|| {
                        // gutter spacing so mouse can still scroll at sides
                        rect()
                            .content(Content::Flex)
                            .width(Size::fill())
                            .margin(Gaps::new(0.0, 10.0, 10.0, 10.0))
                            .child(
                                // stop scrolls to let HTML handle them
                                rect()
                                    .content(Content::Flex)
                                    .width(Size::fill())
                                    .margin(0.0)
                                    .padding(0.0)
                                    .on_wheel(|e: Event<WheelEventData>| e.stop_propagation())
                                    .child(
                                        HtmlViewer::new(HtmlHandle::create(HtmlSource::Html(
                                            fixed_html,
                                        )))
                                        .margin(0.0)
                                        .padding(0.0)
                                        .width(Size::flex(1.0))
                                        .height(Size::px(fixed_size)),
                                    ),
                            )
                    })),
            )
    }
}
