use egui::vec2;

use crate::app::{
    basic_nodes::node_tools,
    editor_graph::{NodePrototype, NodeState, PortPrototype},
};

pub fn exp_prototype() -> NodePrototype {
    NodePrototype {
        name: "Exp".to_string(),
        ports: vec![
            PortPrototype {
                local_position: vec2(0f32, 20f32),
                name: "Inp".to_string(),
                kind: crate::app::editor_graph::PortKindPrototype::Input,
            },
            PortPrototype {
                local_position: vec2(50f32, 20f32),
                name: "".to_string(),
                kind: crate::app::editor_graph::PortKindPrototype::Output(exp_node_eval),
            },
        ],
        state_prototype: NodeState::default(),
        size: vec2(50f32, 50f32),
    }
}

fn exp_node(inp: f32) -> f32 {
    inp.exp()
}

node_tools::node_evaluator! {exp_node, Inp}
