mod api;
mod icons;
mod instance;
mod launcher;
mod mods;
mod style;
mod ui;

use crate::icons::load_icons;
use crate::instance::{Instance, load_instances};
use iced::keyboard::key;
use iced::widget::{button, column, horizontal_rule, horizontal_space, row, scrollable, stack};
use iced::{
    Center, Element, Event, Length, Padding, Size, Subscription, Task, Theme, event, keyboard,
    widget,
};
use log::LevelFilter;
use std::error::Error;
use std::time::SystemTime;

pub fn main() -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Error)
        .level_for("rustic_vs", LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("rustic-vs.log")?)
        .apply()?;

    Ok(iced::application("Rustic", Rustic::update, Rustic::view)
        .subscription(Rustic::subscription)
        .theme(|r: &Rustic| r.theme())
        .window(iced::window::Settings {
            size: Size::new(600.0, 450.0),
            min_size: Some(Size::new(600.0, 450.0)),
            ..iced::window::Settings::default()
        })
        .run()?)
}

#[allow(dead_code)]
struct Rustic {
    dark: bool,
    instances: Vec<Instance>,
    icons: Vec<String>,
    selected_index: Option<usize>,
    selected_icon: Option<String>,
    show_modal: Option<Modal>,
    instance_name: String,
}

#[allow(clippy::enum_variant_names)]
enum Modal {
    NewInstance,
    EditInstance(String),
    ViewInstance,
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
    IconSelected(String),
}

impl Default for Rustic {
    fn default() -> Self {
        Self {
            dark: true,
            instances: load_instances(),
            icons: load_icons(),
            selected_index: None,
            selected_icon: None,
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
                self.selected_index = None;
                Task::none()
            }
            Message::ToggleDark => {
                self.dark = !self.dark;
                Task::none()
            }
            // gui
            Message::SelectInstance(index) => {
                self.selected_index = Some(index);
                self.show_modal = Some(Modal::ViewInstance);
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
                    let mut new_instance = Instance::new(cleaned_name);
                    new_instance.icon = self.selected_icon.clone();

                    new_instance.save();
                    self.instances.push(new_instance);
                    self.hide_modal();

                    self.selected_index = None;
                    self.selected_icon = None;
                }
                Task::none()
            }
            Message::EditInstance(index) => {
                self.selected_index = Some(index);
                self.instance_name = self.instances[index].name.clone();
                self.selected_icon = self.instances[index].icon.clone();
                self.show_modal = Some(Modal::EditInstance(self.instance_name.clone()));
                Task::none()
            }
            Message::EditInstanceSubmit => {
                let cleaned_name = self.instance_name.trim();
                if !cleaned_name.is_empty() {
                    if let Some(index) = self.selected_index {
                        if let Some(instance) = self.instances.get_mut(index) {
                            instance.name = cleaned_name.to_string();
                            instance.icon = self.selected_icon.clone();

                            instance.save();
                            self.hide_modal();

                            self.selected_index = None;
                            self.selected_icon = None;
                        }
                    }
                }
                Task::none()
            }
            // forms
            Message::InstanceName(name) => {
                self.instance_name = name;
                Task::none()
            }
            Message::IconSelected(name) => {
                self.selected_icon = Some(name);
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
                ui::instance_row_base(instance)
                    .push(horizontal_space())
                    .push(button("Play").style(button::primary)),
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
            Some(Modal::ViewInstance) => ui::modal(content, ui::instance_view(self), Message::None),
            Some(Modal::NewInstance) => ui::modal(
                content,
                ui::instance_form(
                    self,
                    "Create new instance",
                    Message::InstanceName,
                    Message::NewInstanceSubmit,
                ),
                Message::None,
            ),
            Some(Modal::EditInstance(instance_name)) => ui::modal(
                content,
                ui::instance_form(
                    self,
                    format!("Edit instance: {}", instance_name),
                    Message::InstanceName,
                    Message::EditInstanceSubmit,
                ),
                Message::None,
            ),
            None => stack![content].into(),
        }
    }
}
