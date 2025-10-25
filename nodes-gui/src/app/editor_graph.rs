use std::collections::HashMap;

use egui::Pos2;

use crate::app::storage::{ID, Storage};

#[derive(Clone)]
pub enum StateValue {
    Float(f32),
    Char(char),
}

type OutputEvaluationFn = fn(
    &NodeWorld,
    &HashMap<String, Option<ID>>,
    &HashMap<String, StateValue>,
    rpds::HashTrieMap<String, f32>,
) -> Option<f32>;

#[derive(Clone)]
pub enum PortKindPrototype {
    Input,
    Output(OutputEvaluationFn),
}

impl PortKindPrototype {
    pub fn instantiate(&self) -> PortKind {
        match self {
            PortKindPrototype::Input => PortKind::Input(None),
            PortKindPrototype::Output(f) => PortKind::Output(*f),
        }
    }
}

#[derive(Clone)]
pub enum PortKind {
    Input(Option<ID>),
    Output(OutputEvaluationFn),
}

impl PortKind {
    pub fn is_input(&self) -> bool {
        match self {
            PortKind::Input(_) => true,
            PortKind::Output(_) => false,
        }
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }
}

#[derive(Clone)]
pub struct Port {
    pub port_info: PortPrototype,
    pub node: ID,
    pub connection_kind: PortKind,
}

#[derive(Clone)]
pub struct PortPrototype {
    pub local_position: egui::Vec2,
    pub name: String,
    pub kind: PortKindPrototype,
}

#[derive(Clone)]
pub struct NodeState {
    pub state: HashMap<String, StateValue>,
    pub render: Option<fn(&mut egui::Ui, &mut HashMap<String, StateValue>, egui::Pos2) -> ()>,
}

// Contains all rendering information for a kind of node.
#[derive(Clone)]
pub struct NodePrototype {
    pub name: String,
    pub ports: Vec<PortPrototype>,
    pub state_prototype: NodeState,
    pub size: egui::Vec2,
}

pub struct Node {
    pub ports: Vec<ID>,

    // In future, should be Rc or id into list of existing prototypes
    pub prototype: NodePrototype,
    pub state: NodeState,

    pub pos: egui::Pos2,
}

pub struct NodeWorld {
    pub nodes: Storage<Node>,
    pub ports: Storage<Port>,
}

impl Default for NodeWorld {
    fn default() -> Self {
        Self {
            nodes: Storage::default(),
            ports: Storage::default(),
        }
    }
}

impl NodeWorld {
    pub fn get_port_pos(&self, id: ID) -> Pos2 {
        self.get_port_pos_from_ref(self.ports.get(id))
    }

    pub fn get_port_pos_from_ref(&self, port: &Port) -> Pos2 {
        self.nodes.get(port.node).pos + port.port_info.local_position
    }

    fn create_port_and_link(&mut self, node_id: ID, port_proto: &PortPrototype) -> ID {
        let new_port = self.ports.create(Port {
            port_info: port_proto.clone(),
            node: node_id,
            connection_kind: port_proto.kind.instantiate(),
        });
        new_port.1
    }

    pub fn create_node(&mut self, pos: Pos2, prototype: &NodePrototype) {
        let new_obj = self
            .nodes
            .create(Node {
                ports: Vec::new(),
                prototype: prototype.clone(),
                state: prototype.state_prototype.clone(),
                pos,
            })
            .1;

        for p in &prototype.ports {
            let new_p = self.create_port_and_link(new_obj, p);
            self.nodes.get_mut(new_obj).ports.push(new_p);
        }
    }

    pub fn evaluate_output_port(&self, id: ID, ctx: rpds::HashTrieMap<String, f32>) -> Option<f32> {
        let port = self.ports.get(id);

        let eval = match &port.connection_kind {
            PortKind::Input(_) => {
                return None;
            }
            PortKind::Output(eval) => eval,
        };

        let mut direct_inputs_map = HashMap::new();
        let node = self.nodes.get(port.node);
        for p in &node.ports {
            let node_port = self.ports.get(*p);
            if let PortKind::Input(i) = &node_port.connection_kind {
                direct_inputs_map.insert(node_port.port_info.name.clone(), *i);
            }
        }

        let node_state = &node.state.state;

        eval(self, &direct_inputs_map, node_state, ctx)
    }
}
