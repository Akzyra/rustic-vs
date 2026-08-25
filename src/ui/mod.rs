use std::path::PathBuf;

use freya::prelude::*;

use crate::core::install::{Install, load_installs};
use crate::core::instance::{Instance, load_instances};

#[derive(PartialEq)]
pub struct Rustic {
    pub cwd: PathBuf,
    pub installs: Vec<Install>,
    pub instances: Vec<Instance>,
    pub selected_instance: usize,
}
impl Rustic {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            installs: Vec::new(),
            instances: Vec::new(),
            selected_instance: 0,
        }
    }

    pub fn reload(&mut self) {
        self.installs = load_installs(&self.cwd);
        self.instances = load_instances(&self.cwd);
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
