use std::collections::HashMap;

use rpds::HashTrieMap;

use crate::app::{
    editor_graph::{NodeWorld, StateValue},
    storage::ID,
};

#[macro_export]
macro_rules! node_evaluator {
    ($f_name: ident, $($arg_name: ident)*) => {

        paste::paste! {
           fn [<$f_name _eval>]

        (
            world: &crate::app::editor_graph::NodeWorld,
            inputs: &std::collections::HashMap<String, Option<crate::app::storage::ID>>,
            _state: &std::collections::HashMap<String, crate::app::editor_graph::StateValue>,
            ctx: rpds::HashTrieMap<String, f32>,
        ) -> Option<f32> {
            Some($f_name ($(crate::app::basic_nodes::node_tools::get_input(stringify!($arg_name), world, inputs, &ctx)?,)*))
        }
    }
    };
}

pub(super) use node_evaluator;

pub fn get_input(
    name: &str,
    world: &NodeWorld,
    inputs: &HashMap<String, Option<ID>>,
    ctx: &HashTrieMap<String, f32>,
) -> Option<f32> {
    match inputs.get(name) {
        Some(Some(id)) => return world.evaluate_output_port(*id, ctx.clone()),
        _ => {}
    };

    ctx.get(name).copied()
}

pub fn get_state_char(name: &str, state: &HashMap<String, StateValue>) -> Option<char> {
    let val = state.get(name)?;
    match val {
        StateValue::Char(c) => Some(*c),
        _ => None,
    }
}

pub fn get_state_char_mut<'a>(
    name: &str,
    state: &'a mut HashMap<String, StateValue>,
) -> Option<&'a mut char> {
    let val = state.get_mut(name)?;
    match val {
        StateValue::Char(c) => Some(c),
        _ => None,
    }
}

pub fn get_state_f32(name: &str, state: &HashMap<String, StateValue>) -> Option<f32> {
    let val = state.get(name)?;
    match val {
        StateValue::Float(c) => Some(*c),
        _ => None,
    }
}

pub fn get_state_f32_mut<'a>(
    name: &str,
    state: &'a mut HashMap<String, StateValue>,
) -> Option<&'a mut f32> {
    let val = state.get_mut(name)?;
    match val {
        StateValue::Float(c) => Some(c),
        _ => None,
    }
}

pub fn get_state_string<'a>(
    name: &str,
    state: &'a HashMap<String, StateValue>,
) -> Option<&'a String> {
    let val = state.get(name)?;
    match val {
        StateValue::String(c) => Some(c),
        _ => None,
    }
}

pub fn get_state_string_mut<'a>(
    name: &str,
    state: &'a mut HashMap<String, StateValue>,
) -> Option<&'a mut String> {
    let val = state.get_mut(name)?;
    match val {
        StateValue::String(c) => Some(c),
        _ => None,
    }
}
