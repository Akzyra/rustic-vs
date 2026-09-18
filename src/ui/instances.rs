use crate::core::instance::Instance;
use crate::ui::components::*;
use freya::i18n::t;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct Instances;

impl Component for Instances {
    fn render(&self) -> impl IntoElement {
        let instances = use_consume::<State<Vec<Instance>>>();

        flex_rect(Direction::Vertical, SPACING_MD)
            .theme_background()
            .padding(SPACING_SM)
            .child(
                rect()
                    .content(Content::Flex)
                    .horizontal()
                    .spacing(SPACING_SM)
                    .child(
                        label()
                            .font_weight(FontWeight::BOLD)
                            .font_size(20.0)
                            .text(t!("instances_header")),
                    )
                    .child(rect().width(Size::flex(1.0)))
                    .child(Button::new().child(t!("add_instance"))),
            )
            .child(
                Table::new()
                    .column_widths([
                        Size::px(100.0),
                        Size::flex(1.0),
                        Size::px(100.0),
                        Size::px(100.0),
                    ])
                    .child(
                        TableHead::new().child(
                            TableRow::new()
                                .child(TableCell::new().child(t!("instances_column_id")))
                                .child(table_cell_fix(
                                    Alignment::Start,
                                    t!("instances_column_name"),
                                ))
                                .child(TableCell::new().child(t!("instances_column_install")))
                                .child(TableCell::new().child(t!("instances_column_actions"))),
                        ),
                    )
                    .child(TableBody::new().child(ScrollView::new().children(
                        instances.read().iter().enumerate().map(|(i, instance)| {
                            TableRow::new()
                                .key(i)
                                .child(TableCell::new().child(instance.id.clone()))
                                .child(table_cell_fix(Alignment::Start, instance.name.clone()))
                                .child(
                                    TableCell::new().child(
                                        instance
                                            .install_id
                                            .clone()
                                            .unwrap_or_else(|| "<unk>".to_string()),
                                    ),
                                )
                                .child(TableCell::new().child("todo"))
                        }),
                    ))),
            )
    }
}
