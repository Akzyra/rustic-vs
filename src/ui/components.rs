use crate::ui::Route;
use freya::prelude::*;

pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 16.0;

pub fn main_rect(header: String) -> Rect {
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
                .text(header),
        )
}

pub fn tool_tipped_pos(
    tooltip: String,
    pos: AttachedPosition,
    child: impl IntoElement,
) -> TooltipContainer {
    TooltipContainer::new(Tooltip::new(tooltip))
        .position(pos)
        .child(child)
}

pub fn tool_tipped(tooltip: String, child: impl IntoElement) -> TooltipContainer {
    tool_tipped_pos(tooltip, AttachedPosition::Bottom, child)
}

pub fn sidebar_link(
    target: Route,
    icon: Bytes,
    title: String,
    tooltip: String,
) -> impl IntoElement {
    ActivableRoute::new(
        target.clone(),
        tool_tipped(
            tooltip,
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
        ),
    )
    .exact(true)
}

pub fn rss_row(title: String, date: String, link: String) -> impl IntoElement {
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
