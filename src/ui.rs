use crate::style;
use iced::Element;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    Container, Row, button, center, column, container, horizontal_rule, horizontal_space,
    mouse_area, opaque, row, stack, text, text_input,
};

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
            // row![
            //     text("Name:"),
            //     text_input("<enter name>", &name_value)
            //         .style(if name_value.trim().is_empty() {
            //             style::text_input_warning
            //         } else {
            //             text_input::default
            //         })
            //         .on_input(name_on_input)
            //         .on_submit(on_submit.clone()),
            // ]
            // .spacing(10)
            // .align_y(Vertical::Center),
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
    .style(style::instance_box)
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
        opaque(mouse_area(center(opaque(modal)).style(style::model_backdrop)).on_press(on_blur))
    ]
    .into()
}
