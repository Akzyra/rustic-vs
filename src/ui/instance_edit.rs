use crate::core::instance::Instance;
use crate::ui::components::*;
use freya::i18n::t;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct InstanceEdit {
    pub instance_id: String,
}

impl Component for InstanceEdit {
    fn render(&self) -> impl IntoElement {
        let instance_id = self.instance_id.clone();
        let instances_state = use_consume::<State<Vec<Instance>>>();
        let instances = instances_state.peek();
        let instance = instances.iter().find(|i| i.id == instance_id).unwrap();

        main_rect(t!("instance_edit_header", name: instance.name.clone())).child("todo: add this..")
    }
}
