use crate::core::rss::{FeedEntry, load_news_feed};
use crate::ui::components::*;
use freya::i18n::t;
use freya::icons::lucide;
use freya::prelude::*;
use std::ops::Not;
use std::time::Duration;

#[derive(PartialEq)]
pub struct News;

impl Component for News {
    fn render(&self) -> impl IntoElement {
        let mut feed = use_consume::<State<Option<Vec<FeedEntry>>>>();
        let mut future = use_future(move || async move {
            if feed.peek().is_none() {
                let entries = load_news_feed().await.unwrap_or_default();
                feed.set(Some(entries));
            }
        });
        let loading = feed.read().is_none();
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
                    .width(Size::fill())
                    .height(Size::fill())
                    .animation(SkeletonAnimation::Shimmer)
                    .duration(Duration::from_secs(2))
                    // needs maybe_child since expect() would fail on None
                    .maybe_child(loading.not().then(|| {
                        ScrollView::new()
                            .width(Size::Fill)
                            .spacing(SPACING_MD)
                            .children(
                                feed.read()
                                    .as_ref()
                                    .expect("maybe_child() guard")
                                    .iter()
                                    .map(rss_row),
                            )
                    })),
            )
    }
}

fn rss_row(feed_entry: &FeedEntry) -> impl IntoElement {
    let short_date = format!("{}", feed_entry.date.format("%Y-%m-%d"));
    Link::new(feed_entry.link.clone()).child(
        Button::new().width(Size::Fill).outline().child(
            rect()
                .content(Content::Flex)
                .horizontal()
                .child(
                    label()
                        .text(feed_entry.title.clone())
                        .font_weight(FontWeight::BOLD)
                        .width(Size::flex(1.0)),
                )
                .child(label().text(short_date)),
        ),
    )
}
