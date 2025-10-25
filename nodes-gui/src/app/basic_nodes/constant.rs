use std::collections::HashMap;

use egui::{Pos2, Rect, vec2};
use rpds::HashTrieMap;

use crate::app::{
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
    world: &NodeWorld,
    inps: &HashMap<String, Option<ID>>,
    state: &HashMap<String, StateValue>,
    ctx: HashTrieMap<String, f32>,
) -> Option<f32> {
    match state.get("val") {
        Some(StateValue::Float(f)) => Some(*f),
        _ => None,
    }
}

fn render_constant_node(ui: &mut egui::Ui, state: &mut HashMap<String, StateValue>, pos: Pos2) {
    let val = match state.get_mut("val").unwrap() {
        StateValue::Float(f) => f,
        _ => panic!(),
    };
    let widget = egui::Slider::new(val, -10f32..=10f32).clamping(egui::SliderClamping::Never);
    ui.put(
        Rect::from_min_size(pos + vec2(0f32, 50f32), vec2(30f32, 15f32)),
        widget,
    );
    //ui.put(Rect::from_min_size(pos, vec2(100f32, 100f32)), widget);
}
