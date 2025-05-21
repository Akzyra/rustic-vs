mod api;
mod instance;
mod launcher;
mod style;

use crate::instance::{Instance, load_instances};
use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::key;
use iced::widget::{
    button, center, column, container, horizontal_rule, horizontal_space, image, mouse_area,
    opaque, row, scrollable, stack, text, text_input,
};
use iced::{
    Center, Element, Event, Length, Padding, Size, Subscription, Task, Theme, event, keyboard,
    widget,
};

pub fn main() -> iced::Result {
    iced::application("Rustic", Rustic::update, Rustic::view)
        .subscription(Rustic::subscription)
        .theme(|r: &Rustic| r.theme())
        .window(iced::window::Settings {
            size: Size::new(600.0, 450.0),
            min_size: Some(Size::new(600.0, 450.0)),
            ..iced::window::Settings::default()
        })
        .run()
}

struct Rustic {
    dark: bool,
    instances: Vec<Instance>,
    show_new_instance: bool,
    instance_name: String,
}

#[derive(Debug, Clone)]
enum Message {
    None,
    Event(Event),
    Refresh,
    ToggleDark,
    HideModal,
    NewInstance,
    NewInstanceName(String),
    NewInstanceSubmit,
}

impl Default for Rustic {
    fn default() -> Self {
        Self {
            dark: true,
            instances: load_instances(),
            show_new_instance: false,
            instance_name: String::new(),
        }
    }
}

impl Rustic {
    fn theme(&self) -> Theme {
        if self.dark { Theme::Dark } else { Theme::Light }
    }

    fn hide_modal(&mut self) {
        self.show_new_instance = false;
        self.instance_name.clear();
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    if self.show_new_instance {
                        self.hide_modal();
                    }
                    Task::none()
                }
                _ => Task::none(),
            },
            Message::Refresh => {
                self.instances = load_instances();
                Task::none()
            }
            Message::ToggleDark => {
                self.dark = !self.dark;
                Task::none()
            }
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
            Message::NewInstance => {
                self.show_new_instance = true;
                widget::focus_next()
            }
            Message::NewInstanceName(name) => {
                self.instance_name = name;
                Task::none()
            }
            Message::NewInstanceSubmit => {
                // TODO: new instance
                if !self.instance_name.trim().is_empty() {
                    let cleaned_name = self.instance_name.trim();
                    let new_instance = Instance::new(cleaned_name);
                    new_instance.save();
                    self.instances.push(new_instance);
                    self.hide_modal();
                }
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn view(&self) -> Element<Message> {
        let menu = row![
            button("New Instance")
                .style(button::primary)
                .on_press(Message::NewInstance),
            button("Folders").style(button::secondary), // TODO: dropdown
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
                    row![
                        button("Mods").style(button::secondary),
                        button("Edit").style(button::secondary),
                        button("Play").style(button::primary),
                    ]
                    .spacing(5)
                ]
                .spacing(10)
                .align_y(Vertical::Center),
            )
            .style(style::instance_box)
            .width(Length::Fill)
            .padding(Padding::from(5).right(10))
        });

        let instance_list = column(instance_widgets.map(Element::from))
            .spacing(10)
            .padding(10);

        let content = column![
            menu.padding(10),
            horizontal_rule(1),
            scrollable(instance_list).spacing(0),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        if self.show_new_instance {
            let form_new_instance = container(column![
                row![text("Create new instance").size(20)]
                    .padding(10)
                    .spacing(10),
                horizontal_rule(1),
                column![
                    row![
                        text("Name:"),
                        text_input("<enter name>", &self.instance_name)
                            .style(if self.instance_name.trim().is_empty() {
                                style::text_input_warning
                            } else {
                                text_input::default
                            })
                            .on_input(Message::NewInstanceName)
                            .on_submit(Message::NewInstanceSubmit),
                    ]
                    .spacing(10)
                    .align_y(Vertical::Center),
                    row![
                        horizontal_space(),
                        button(text("OK").align_x(Horizontal::Center))
                            .width(90)
                            .on_press(Message::NewInstanceSubmit),
                        button(text("Cancel").align_x(Horizontal::Center))
                            .width(90)
                            .style(button::secondary)
                            .on_press(Message::HideModal),
                    ]
                    .spacing(10)
                ]
                .padding(10)
                .spacing(10),
            ])
            .width(300)
            .style(style::instance_box);

            modal(content, form_new_instance, Message::None)
        } else {
            content.into()
        }
    }
}

// based on https://github.com/iced-rs/iced/blob/master/examples/modal/src/main.rs
fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    modal: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(mouse_area(center(opaque(modal)).style(style::model_backdrop)).on_press(on_blur))
    ]
    .into()
}
