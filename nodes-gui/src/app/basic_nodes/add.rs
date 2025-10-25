use std::collections::HashMap;

use egui::Pos2;
use rpds::HashTrieMap;

use crate::app::{
    basic_nodes::node_tools::{get_input, get_state_char, get_state_char_mut},
    editor_graph::{
        NodePrototype, NodeState, NodeWorld, PortKindPrototype, PortPrototype, StateValue,
    },
    storage::ID,
};

pub fn add_node_prototype() -> NodePrototype {
    NodePrototype {
        name: "Binary Math".to_string(),
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
            state: HashMap::from([("op".to_string(), StateValue::Char('+'))]),
            render: Some(add_render),
        },
    }
}

fn add_render(ui: &mut egui::Ui, state: &mut HashMap<String, StateValue>, _: Pos2) -> bool {
    let val = get_state_char_mut("op", state).unwrap();
    egui::containers::ComboBox::from_id_salt("test_box")
        .width(10f32)
        .selected_text(format!("{}", *val))
        .show_ui(ui, |ui| {
            ui.selectable_value(val, '+', "Add");
            ui.selectable_value(val, '-', "Sub");
            ui.selectable_value(val, '*', "Mul");
        })
        .response
        .changed()
}

fn add_node_eval(
    world: &NodeWorld,
    inputs: &HashMap<String, Option<ID>>,
    state: &HashMap<String, StateValue>,
    ctx: HashTrieMap<String, f32>,
) -> Option<f32> {
    let first_f = get_input("A", world, inputs, &ctx)?;
    let second_f = get_input("B", world, inputs, &ctx)?;
    let op = get_state_char("op", state)?;

    match op {
        '+' => Some(first_f + second_f),
        '-' => Some(first_f - second_f),
        '*' => Some(first_f * second_f),
        _ => None,
    }
}
