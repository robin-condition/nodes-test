use std::collections::HashMap;

use rpds::HashTrieMap;

use crate::app::{
    editor_graph::{
        NodePrototype, NodeState, NodeWorld, PortKindPrototype, PortPrototype, StateValue,
    },
    storage::ID,
};

pub fn add_node_prototype() -> NodePrototype {
    NodePrototype {
        name: "Add Float".to_string(),
        size: egui::vec2(100f32, 80f32),
        ports: vec![
            PortPrototype {
                local_position: egui::vec2(0f32, 50f32),
                name: "A".to_string(),
                kind: PortKindPrototype::Input,
            },
            PortPrototype {
                local_position: egui::Vec2 { x: 0f32, y: 60f32 },
                name: "B".to_string(),
                kind: PortKindPrototype::Input,
            },
            PortPrototype {
                local_position: egui::vec2(100f32, 40f32),
                name: "Out".to_string(),
                kind: PortKindPrototype::Output(add_node_eval),
            },
        ],
        state_prototype: NodeState {
            state: HashMap::new(),
            render: None,
        },
    }
}

fn add_node_eval(
    world: &NodeWorld,
    inputs: &HashMap<String, Option<ID>>,
    _: &HashMap<String, StateValue>,
    ctx: HashTrieMap<String, f32>,
) -> Option<f32> {
    let first_f = inputs
        .get("First")
        .copied()
        .flatten()
        .map(|p| world.evaluate_output_port(p, ctx.clone()))
        .flatten()
        .or(ctx.get("First").copied())?;
    let second_f = inputs
        .get("Second")
        .copied()
        .flatten()
        .map(|p| world.evaluate_output_port(p, ctx.clone()))
        .flatten()
        .or(ctx.get("Second").copied())?;
    Some(first_f + second_f)
}
