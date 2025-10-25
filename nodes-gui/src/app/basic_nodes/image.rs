use std::collections::HashMap;

use egui::vec2;

use crate::app::editor_graph::{NodePrototype, NodeState, PortPrototype};

pub fn done_node() -> NodePrototype {
    NodePrototype {
        name: "Out".to_string(),
        ports: vec![PortPrototype {
            local_position: vec2(0f32, 10f32),
            name: "Inp".to_string(),
            kind: crate::app::editor_graph::PortKindPrototype::Input,
        }],
        state_prototype: NodeState {
            state: HashMap::new(),
            render: None,
        },
        size: vec2(50f32, 50f32),
    }
}
