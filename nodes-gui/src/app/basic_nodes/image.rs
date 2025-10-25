use std::collections::HashMap;

use egui::{Pos2, Rect, vec2};
use rpds::HashTrieMap;

use crate::app::{
    editor_graph::{NodePrototype, NodeState, NodeWorld, PortPrototype, StateValue},
    storage::ID,
};
