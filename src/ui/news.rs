use crate::core::rss::load_news_feed;
use crate::ui::components::*;
use freya::i18n::t;
use freya::prelude::*;
use rss::Channel;

#[derive(PartialEq)]
pub struct News;

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
                    let title = item.title?;
                    let link = item.link?;
                    let date = item.pub_date?;

                    let short_date = chrono::DateTime::parse_from_rfc2822(&*date)
                        .map(|dt| format!("{}", dt.format("%Y-%m-%d")))
                        .unwrap_or(date);

                    Some(rss_row(title, short_date, link).into_element())
                }))
                .into_element(),
        })
    }
}

fn rss_row(title: String, date: String, link: String) -> impl IntoElement {
    Link::new(link).child(
        Button::new().width(Size::Fill).outline().child(
            rect()
                .content(Content::Flex)
                .horizontal()
                .child(
                    label()
                        .text(title)
                        .font_weight(FontWeight::BOLD)
                        .width(Size::flex(1.0)),
                )
                .child(label().text(date)),
        ),
    )
}
