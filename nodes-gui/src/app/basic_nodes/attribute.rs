use std::collections::HashMap;

use egui::{Pos2, vec2};
use rpds::HashTrieMap;

use crate::app::{
    basic_nodes::node_tools::{get_state_string, get_state_string_mut},
    editor_graph::{
        NodePrototype, NodeState, NodeWorld, PortKindPrototype, PortPrototype, StateValue,
    },
    storage::ID,
};

pub fn attribute_prototype() -> NodePrototype {
    NodePrototype {
        name: "Attr".to_string(),
        ports: vec![PortPrototype {
            local_position: vec2(100f32, 50f32),
            name: "".to_string(),
            kind: PortKindPrototype::Output(eval_attr),
        }],
        state_prototype: NodeState {
            state: HashMap::from([("name".to_string(), StateValue::String("".to_string()))]),
            render: Some(render_attr),
        },
        size: vec2(100f32, 70f32),
    }
}

fn render_attr(ui: &mut egui::Ui, state: &mut HashMap<String, StateValue>, _: Pos2) -> bool {
    ui.text_edit_singleline(get_state_string_mut("name", state).unwrap())
        .changed()
}

fn eval_attr(
    _: &NodeWorld,
    _: &HashMap<String, Option<ID>>,
    state: &HashMap<String, StateValue>,
    ctx: HashTrieMap<String, f32>,
) -> Option<f32> {
    let name = get_state_string("name", state)?;
    let val = ctx.get(name)?;
    Some(*val)
}
