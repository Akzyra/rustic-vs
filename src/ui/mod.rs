use freya::prelude::*;

use crate::core::install::Install;
use crate::core::instance::Instance;

#[derive(PartialEq)]
pub struct Rustic {
    pub installs: Vec<Install>,
    pub instances: Vec<Instance>,
    pub selected_instance: usize,
}
impl Rustic {
    pub fn new() -> Self {
        Self {
            installs: Vec::new(),
            instances: Vec::new(),
            selected_instance: 0,
        }
    }
}
impl App for Rustic {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .direction(Direction::Horizontal)
            .child(sidebar())
            .child(root())
    }
}

fn sidebar() -> impl IntoElement {
    rect()
        .width(Size::px(250.0))
        .height(Size::fill())
        .background(Color::DARK_GRAY.with_a(16))
        .padding(Gaps::new_all(10.0))
        .direction(Direction::Vertical)
        .child("Sidebar")
        .child(SideBarItem::new().child("Tab 1"))
        .child(SideBarItem::new().child("Tab 2"))
        .child(SideBarItem::new().child("Tab 3"))
}

fn root() -> impl IntoElement {
    rect()
        .width(Size::fill())
        .height(Size::fill())
        .padding(Gaps::new_all(10.0))
        .direction(Direction::Vertical)
        .child("ROOT")
}
