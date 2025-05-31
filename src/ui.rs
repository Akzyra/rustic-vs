use crate::icons::load_icon;
use crate::instance::Instance;
use crate::{Message, Rustic, style};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    Container, Row, button, center, column, container, horizontal_rule, horizontal_space, image,
    mouse_area, opaque, radio, row, scrollable, stack, text, text_input,
};
use iced::{Element, Length};
use std::convert::Into;

pub fn form_text_input<'a, Message>(
    label: impl text::IntoFragment<'a>,
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
) -> Row<'a, Message>
where
    Message: Clone + 'a,
{
    form_row(
        label,
        text_input(placeholder, value)
            .style(if value.trim().is_empty() {
                style::text_input_warning
            } else {
                text_input::default
            })
            .on_input(on_input)
            .on_submit(on_submit)
            .into(),
    )
    .spacing(10)
    .align_y(Vertical::Center)
}

pub fn form_row<'a, Message>(
    label: impl text::IntoFragment<'a>,
    widget: Element<'a, Message>,
) -> Row<'a, Message>
where
    Message: Clone + 'a,
{
    row![text(label).width(60), widget]
        .spacing(10)
        .align_y(Vertical::Center)
}

pub fn instance_form<'a>(
    state: &Rustic,
    title: impl text::IntoFragment<'a>,
    name_on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
) -> Container<'a, Message> {
    container(column![
        row![text(title).size(20)].padding(10).spacing(10),
        horizontal_rule(1),
        column![
            form_text_input(
                "Name:",
                "<enter name>",
                &state.instance_name,
                name_on_input,
                on_submit.clone()
            ),
            form_row(
                "Icon:",
                scrollable(
                    column(state.icons.clone().into_iter().map(|icon| {
                        radio(
                            icon.to_string(),
                            &icon,
                            state.selected_icon.clone().as_ref(),
                            |s| Message::IconSelected(s.to_string()),
                        )
                        .into()
                    }))
                    .spacing(5)
                    .padding(5)
                )
                .style(|t, s| {
                    scrollable::Style {
                        container: style::rounded_container(t),
                        ..scrollable::default(t, s)
                    }
                })
                .spacing(5)
                .width(Length::Fill)
                .into()
            )
            .height(75),
            row![
                horizontal_space(),
                button(text("OK").align_x(Horizontal::Center))
                    .width(90)
                    .on_press(on_submit),
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
    .style(style::rounded_container)
}

pub fn instance_row_base(instance: &Instance) -> Row<crate::Message> {
    row![
        Element::from(image(load_icon(&instance.icon)).width(48).height(48)),
        column![
            text(instance.name.clone()).size(16),
            row![
                text(format!("{} mods", instance.mods_count())).size(12),
                text("•").size(12),
                text(format!("Folder: {}", instance.folder_name_string())).size(12),
            ]
            .spacing(5)
            .align_y(Vertical::Center)
        ]
        .spacing(5),
    ]
    .spacing(10)
    .align_y(Vertical::Center)
}

pub fn instance_view(state: &Rustic) -> Container<crate::Message> {
    let index = state.selected_index.expect("modal open without selection");
    let instance = &state.instances[index];

    container(column![
        instance_row_base(instance)
            .push(horizontal_space())
            .push(
                button("Edit")
                    .style(button::secondary)
                    .on_press(crate::Message::EditInstance(index))
            )
            .push(
                button("X")
                    .style(button::secondary)
                    .on_press(crate::Message::HideModal)
            )
            .padding(10),
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
            .push_maybe(instance.mods.is_empty().then(|| {
                text("no mods found")
                    .height(50)
                    .width(Length::Fill)
                    .align_y(Vertical::Center)
                    .align_x(Horizontal::Center)
            }))
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
