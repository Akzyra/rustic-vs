mod api;
mod instance;
mod launcher;

use crate::instance::{Instance, load_instances};
use iced::alignment::Vertical;
use iced::widget::{button, container, horizontal_space, scrollable, text};
use iced::widget::{column, image, row};
use iced::{Center, Element, Length, Padding, Size, Subscription, Theme};

pub fn main() -> iced::Result {
    iced::application("Rustic", Rustic::update, Rustic::view)
        .subscription(Rustic::subscription)
        .theme(|r: &Rustic| r.theme())
        .window(iced::window::Settings {
            size: Size::new(500.0, 450.0),
            min_size: Some(Size::new(500.0, 450.0)),
            ..iced::window::Settings::default()
        })
        .run()
}

struct Rustic {
    dark: bool,
    instances: Vec<Instance>,
}

#[derive(Debug, Clone)]
enum Message {
    Refresh,
    ToggleDark,
}

impl Default for Rustic {
    fn default() -> Self {
        Self {
            dark: true,
            instances: load_instances(),
        }
    }
}

impl Rustic {
    fn theme(&self) -> Theme {
        if self.dark { Theme::Dark } else { Theme::Light }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Refresh => self.instances = load_instances(),
            Message::ToggleDark => self.dark = !self.dark,
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Message> {
        let menu = row![
            button("New Instance").style(button::primary),
            button("Refresh")
                .on_press(Message::Refresh)
                .style(button::secondary),
            horizontal_space(),
            button("L/D")
                .on_press(Message::ToggleDark)
                .style(button::secondary),
            button("Settings").style(button::danger),
        ]
        .spacing(10)
        .align_y(Center);

        let instance_widgets = self.instances.iter().map(|instance| {
            container(
                row![
                    Element::from(image("icons/vs.png").width(48).height(48)),
                    column![
                        text(instance.name.clone()).size(16),
                        text("X mods").size(12),
                    ]
                    .spacing(5),
                    horizontal_space(),
                    row![button("Mods"), button("Edit"),].spacing(5)
                ]
                .spacing(10)
                .align_y(Vertical::Center),
            )
            .style(container::bordered_box)
            .width(Length::Fill)
            .padding(10)
        });

        let instance_list = column(instance_widgets.map(Element::from))
            .spacing(10)
            .padding(Padding::ZERO.bottom(2)); // fix last instance cut-off

        column![menu, scrollable(instance_list).spacing(10)]
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
