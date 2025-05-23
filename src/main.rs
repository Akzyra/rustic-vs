mod api;
mod instance;
mod launcher;
mod style;
mod ui;

use crate::instance::{Instance, load_instances};
use iced::alignment::Vertical;
use iced::keyboard::key;
use iced::widget::{
    button, column, horizontal_rule, horizontal_space, image, row, scrollable, text,
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
    selected_instance: Option<usize>,
    show_modal: Option<Modal>,
    instance_name: String,
}

enum Modal {
    NewInstance,
    EditInstance(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    // misc
    None,
    Event(Event),
    Refresh,
    ToggleDark,
    // gui
    SelectInstance(usize),
    // modals
    HideModal,
    NewInstance,
    NewInstanceSubmit,
    EditInstance(usize),
    EditInstanceSubmit,
    // form fields
    InstanceName(String),
}

impl Default for Rustic {
    fn default() -> Self {
        Self {
            dark: true,
            instances: load_instances(),
            selected_instance: None,
            show_modal: None,
            instance_name: String::new(),
        }
    }
}

impl Rustic {
    fn theme(&self) -> Theme {
        if self.dark { Theme::Dark } else { Theme::Light }
    }

    fn hide_modal(&mut self) {
        self.show_modal = None;
        self.instance_name.clear();
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // misc
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
                    if self.show_modal.is_some() {
                        self.hide_modal();
                    }
                    Task::none()
                }
                _ => Task::none(),
            },
            Message::Refresh => {
                self.instances = load_instances();
                self.selected_instance = None;
                Task::none()
            }
            Message::ToggleDark => {
                self.dark = !self.dark;
                Task::none()
            }
            // gui
            Message::SelectInstance(index) => {
                self.selected_instance = Some(index);
                Task::none()
            }
            // modals
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
            Message::NewInstance => {
                self.show_modal = Some(Modal::NewInstance);
                widget::focus_next()
            }
            Message::NewInstanceSubmit => {
                let cleaned_name = self.instance_name.trim();
                if !cleaned_name.is_empty() {
                    let new_instance = Instance::new(cleaned_name);
                    new_instance.save();
                    self.instances.push(new_instance);
                    self.hide_modal();
                }
                Task::none()
            }
            Message::EditInstance(index) => {
                self.selected_instance = Some(index);
                self.instance_name = self.instances[index].name.clone();
                self.show_modal = Some(Modal::EditInstance(self.instance_name.clone()));
                Task::none()
            }
            Message::EditInstanceSubmit => {
                let cleaned_name = self.instance_name.trim();
                if !cleaned_name.is_empty() {
                    if let Some(index) = self.selected_instance {
                        let instance = self.instances.get_mut(index).expect("should exist");
                        instance.name = cleaned_name.to_string();
                        instance.save();
                        self.hide_modal();
                        self.selected_instance = None;
                    }
                }
                Task::none()
            }
            // forms
            Message::InstanceName(name) => {
                self.instance_name = name;
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

        let instance_widgets = self.instances.iter().enumerate().map(|(index, instance)| {
            button(
                row![
                    Element::from(image(ui::load_icon(&instance.icon)).width(48).height(48)),
                    column![
                        text(instance.name.clone()).size(16),
                        text("X mods").size(12),
                    ]
                    .spacing(5),
                    horizontal_space(),
                    row![
                        button("Mods").style(button::secondary),
                        button("Edit")
                            .style(button::secondary)
                            .on_press(Message::EditInstance(index)),
                        button("Play").style(button::primary),
                    ]
                    .spacing(5)
                ]
                .spacing(10)
                .align_y(Vertical::Center),
            )
            .style(style::instance_button)
            .width(Length::Fill)
            .padding(Padding::from(5).right(10))
            .on_press(Message::SelectInstance(index))
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

        match &self.show_modal {
            Some(Modal::NewInstance) => ui::modal(
                content,
                ui::instance_form(
                    "Create new instance",
                    &self.instance_name,
                    Message::InstanceName,
                    Message::NewInstanceSubmit,
                    Message::HideModal,
                ),
                Message::None,
            ),
            Some(Modal::EditInstance(instance_name)) => ui::modal(
                content,
                ui::instance_form(
                    format!("Edit instance: {}", instance_name),
                    &self.instance_name,
                    Message::InstanceName,
                    Message::EditInstanceSubmit,
                    Message::HideModal,
                ),
                Message::None,
            ),
            None => content.into(),
        }
    }
}
