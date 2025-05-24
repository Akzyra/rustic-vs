use crate::{Rustic, style};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    Container, Row, button, center, column, container, horizontal_rule, horizontal_space, image,
    mouse_area, opaque, row, scrollable, stack, text, text_input,
};
use iced::{Element, Length};
use std::convert::Into;
use std::env;

pub const DEFAULT_ICON: &[u8] = include_bytes!("../assets/default.png");

pub fn load_icon(name: &Option<String>) -> image::Handle {
    if let Some(name) = name {
        let root = env::current_dir().expect("Failed to get CWD");
        let path = root.join("icons").join(name);
        if path.is_file() {
            return path.strip_prefix(root).expect("failed rel path").into();
        }
    }
    image::Handle::from_bytes(DEFAULT_ICON)
}

pub fn form_text_input<'a, Message>(
    label: impl text::IntoFragment<'a>,
    placeholder: &str,
    value: &String,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
) -> Row<'a, Message>
where
    Message: Clone + 'a,
{
    row![
        text(label),
        text_input(placeholder, &value)
            .style(if value.trim().is_empty() {
                style::text_input_warning
            } else {
                text_input::default
            })
            .on_input(on_input)
            .on_submit(on_submit),
    ]
    .spacing(10)
    .align_y(Vertical::Center)
}

pub fn instance_form<'a, Message>(
    title: impl text::IntoFragment<'a>,
    name_value: &'a String,
    name_on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
    on_cancel: Message,
) -> Container<'a, Message>
where
    Message: Clone + 'a,
{
    container(column![
        row![text(title).size(20)].padding(10).spacing(10),
        horizontal_rule(1),
        column![
            form_text_input(
                "Name:",
                "<enter name>",
                &name_value,
                name_on_input,
                on_submit.clone()
            ),
            row![
                horizontal_space(),
                button(text("OK").align_x(Horizontal::Center))
                    .width(90)
                    .on_press(on_submit),
                button(text("Cancel").align_x(Horizontal::Center))
                    .width(90)
                    .style(button::secondary)
                    .on_press(on_cancel),
            ]
            .spacing(10)
        ]
        .padding(10)
        .spacing(10),
    ])
    .width(300)
    .style(style::rounded_container)
}

pub fn instance_view(state: &Rustic) -> Container<crate::Message> {
    let index = state.selected_index.expect("modal open without selection");
    let instance = &state.instances[index];
    let folder_name = instance.folder_name.to_string_lossy().to_string();

    container(column![
        row![
            Element::from(image(load_icon(&instance.icon)).width(48).height(48)),
            column![
                text(instance.name.clone()).size(16),
                row![
                    text(format!("{} mods", instance.mods.len())).size(12),
                    text("•").size(12),
                    text(format!("Folder: {folder_name}")).size(12),
                ]
                .spacing(5)
                .align_y(Vertical::Center)
            ]
            .spacing(5),
            horizontal_space(),
            button("Edit")
                .style(button::secondary)
                .on_press(crate::Message::EditInstance(index)),
            button("X")
                .style(button::secondary)
                .on_press(crate::Message::HideModal)
        ]
        .align_y(Vertical::Center)
        .padding(10)
        .spacing(10),
        horizontal_rule(1),
        scrollable(
            column(
                instance
                    .mods
                    .iter()
                    .enumerate()
                    .map(|(index, mod_info)| {
                        container(if mod_info.name.is_empty() {
                            row![
                                text(mod_info.zip_name.to_string_lossy()).width(Length::Fill),
                                text("<parse error>").size(12),
                            ]
                            .spacing(10)
                            .align_y(Vertical::Top)
                        } else {
                            row![
                                text(&mod_info.name).width(Length::FillPortion(1)),
                                text(&mod_info.version).width(60),
                                text(norm_str(&mod_info.description)).width(Length::FillPortion(2)),
                            ]
                            .spacing(10)
                            .align_y(Vertical::Top)
                        })
                        .padding([5, 10])
                        .style(style::striped(index))
                    })
                    .map(Element::from),
            )
            .width(Length::Fill)
            .padding(10)
        )
        .spacing(0)
    ])
    .width(Length::Fill)
    .style(style::rounded_container)
}

// based on https://github.com/iced-rs/iced/blob/master/examples/modal/src/main.rs
pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    modal: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(
                center(opaque(modal))
                    .padding(40)
                    .style(style::model_backdrop)
            )
            .on_press(on_blur)
        )
    ]
    .into()
}

pub fn norm_str(str: &str) -> String {
    str.replace("\t", "    ")
}
