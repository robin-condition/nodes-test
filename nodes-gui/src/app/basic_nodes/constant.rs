use std::collections::HashMap;

use egui::{Pos2, Rect, vec2};
use rpds::HashTrieMap;

use crate::app::{
    basic_nodes::node_tools::{get_state_f32, get_state_f32_mut},
    editor_graph::{NodePrototype, NodeState, NodeWorld, PortPrototype, StateValue},
    storage::ID,
};

pub fn constant_node_prototype() -> NodePrototype {
    NodePrototype {
        name: "Constant".to_string(),
        ports: vec![PortPrototype {
            local_position: vec2(200f32, 30f32),
            name: "".to_string(),
            kind: crate::app::editor_graph::PortKindPrototype::Output(evaluate_constant_node),
        }],
        state_prototype: NodeState {
            state: HashMap::from([("val".to_string(), StateValue::Float(1f32))]),
            render: Some(render_constant_node),
        },
        size: vec2(200f32, 100f32),
    }
}

fn evaluate_constant_node(
    _: &NodeWorld,
    _: &HashMap<String, Option<ID>>,
    state: &HashMap<String, StateValue>,
    _: HashTrieMap<String, f32>,
) -> Option<f32> {
    get_state_f32("val", state)
}

fn render_constant_node(
    ui: &mut egui::Ui,
    state: &mut HashMap<String, StateValue>,
    pos: Pos2,
) -> bool {
    let val = get_state_f32_mut("val", state).unwrap();
    let widget = egui::Slider::new(val, 0f32..=1f32).clamping(egui::SliderClamping::Never);
    ui.put(
        Rect::from_min_size(pos + vec2(0f32, 50f32), vec2(30f32, 15f32)),
        widget,
    )
    .changed()
    //ui.put(Rect::from_min_size(pos, vec2(100f32, 100f32)), widget);
}
