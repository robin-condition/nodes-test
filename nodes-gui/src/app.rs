// https://github.com/emilk/eframe_template/blob/main/src/app.rs

use std::{collections::HashMap, process::Output};

use egui::{
    Color32, CornerRadius, FontId, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Vec2,
    emath::TSTransform,
    epaint::{CircleShape, CornerRadiusF32, PathShape, PathStroke, RectShape, TextShape},
    text::Fonts,
    vec2,
};

pub struct App {
    // Example stuff:
    label: String,

    value: f32,

    state: UIState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            state: UIState {
                world: Default::default(),
                interacting_mode: InteractingMode::Idle,
                view: TSTransform::IDENTITY,
            },
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            draw_node(ui, &mut self.state);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(usize);

#[derive(Clone)]
enum SourceRef {
    Connector(usize),
    OutputPortRef(NodeId, usize),
}

struct Connector {
    pos: egui::Vec2,
    connection: Option<SourceRef>,
}

#[derive(Clone)]
struct InputPort {
    local_position: egui::Vec2,
    name: String,
}

#[derive(Clone)]
struct OutputPort {
    local_position: egui::Vec2,
    name: String,
}

// Contains all rendering information for a kind of node.
#[derive(Clone)]
struct NodePrototype {
    name: String,
    inputs: Vec<InputPort>,
    outputs: Vec<OutputPort>,
    size: egui::Vec2,
}

struct Node {
    id: NodeId,
    connected_inputs: Vec<Option<SourceRef>>,

    // In future, should be Rc or id into list of existing prototypes
    prototype: NodePrototype,

    pos: egui::Pos2,
}

struct NodeWorld {
    nodes: Vec<Node>,
    unused_ids: Vec<NodeId>,
    next_unallocated_id: NodeId,
    ids_to_inds: HashMap<NodeId, usize>,

    // Just for funsies
    lines: Vec<(Pos2, Pos2)>,
}

impl Default for NodeWorld {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            unused_ids: Default::default(),
            lines: Default::default(),
            ids_to_inds: Default::default(),
            next_unallocated_id: NodeId(0),
        }
    }
}

impl NodeWorld {
    fn get_next_id(&mut self) -> NodeId {
        if let Some(id) = self.unused_ids.pop() {
            return id;
        }

        let new_id = self.next_unallocated_id;
        self.next_unallocated_id = NodeId(self.next_unallocated_id.0 + 1);
        return new_id;
    }

    pub fn create_node(&mut self, pos: Pos2, prototype: &NodePrototype) -> &Node {
        let id = self.get_next_id();
        let new_node = Node {
            id,
            connected_inputs: vec![Default::default(); prototype.inputs.len()],
            prototype: prototype.clone(),
            pos,
        };
        self.nodes.push(new_node);
        let ind = self.nodes.len() - 1;
        self.ids_to_inds.insert(id, ind);
        &self.nodes[ind]
    }

    pub fn remove_node(&mut self, id: NodeId) {
        let ind = self.ids_to_inds.remove(&id).unwrap();
        self.unused_ids.push(id);

        if ind == self.nodes.len() - 1 {
            self.nodes.pop();
            return;
        }

        let node_to_move = self.nodes.pop().unwrap();
        let id_to_update = node_to_move.id;
        self.ids_to_inds.insert(id_to_update, ind);
        self.nodes[ind] = node_to_move;
    }

    pub fn get_node(&self, id: NodeId) -> &Node {
        &self.nodes[*self.ids_to_inds.get(&id).unwrap()]
    }

    pub fn get_mut_node(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[*self.ids_to_inds.get(&id).unwrap()]
    }

    pub fn node_exists(&self, id: NodeId) -> bool {
        self.ids_to_inds.contains_key(&id)
    }
}

enum InteractingMode {
    Idle,
    // Temp, just for funsies
    DrawingLine(Pos2),
    Panning,
}

struct UIState {
    world: NodeWorld,
    view: TSTransform,
    interacting_mode: InteractingMode,
}

struct DrawingState {
    lines: Vec<Shape>,
    other_shapes: Vec<Shape>,
}

impl UIState {
    pub fn act(
        &mut self,
        ui: &mut egui::Ui,
        response: &mut Response,
        drawing_state: &mut DrawingState,
    ) {
        match &self.interacting_mode {
            InteractingMode::Idle => {
                if response.drag_started() {
                    let ctrl = ui.input(|i| i.modifiers.ctrl);
                    if let (true, Some(p)) = (ctrl, response.interact_pointer_pos()) {
                        let worldspace = self.view.inverse() * p;
                        self.interacting_mode = InteractingMode::DrawingLine(worldspace);
                        //println!("Drag Started: {worldspace}");
                    } else {
                        self.interacting_mode = InteractingMode::Panning;
                    }
                }
            }
            InteractingMode::DrawingLine(pos2) => {
                if response.drag_stopped() {
                    if let Some(end_ui_pos) = response.interact_pointer_pos() {
                        let end_pos = self.view.inverse() * end_ui_pos;

                        self.world.lines.push((*pos2, end_pos));
                    }
                    self.interacting_mode = InteractingMode::Idle;
                    //println!("Drag stopped!");
                } else if response.dragged() {
                    if let Some(end_ui_pos) = response.interact_pointer_pos() {
                        let end_pos = self.view.inverse() * end_ui_pos;

                        //println!("Drawing from {} to {}", start_pos, end_pos);

                        draw_line(
                            &mut drawing_state.lines,
                            *pos2,
                            end_pos,
                            100usize,
                            &self.view,
                        );
                    }
                }
            }
            InteractingMode::Panning => {
                if response.drag_stopped() {
                    self.interacting_mode = InteractingMode::Idle;
                }
                let delt = response.drag_delta();
                if delt.x != 0f32 || delt.y != 0f32 {
                    // This happens after view because it's a screen-space transformation.
                    self.view = TSTransform::from_translation(delt) * self.view;
                }
            }
        }
    }
}

fn add_node(ui_state: &mut UIState) {}

// Derived myself :)
// From:
// f(0)=0, f(1)=1
// f'(0)=f'(1)=0
// f''(0)=f''(1)=0
// Then I googled it and found Knuth proposed it apparently
fn smoother_step(t: f32) -> f32 {
    ((6f32 * t - 15f32) * t + 10f32) * t.powi(3)
}

fn draw_line(
    lines: &mut Vec<Shape>,
    start_pt: Pos2,
    end_pt: Pos2,
    steps: usize,
    view: &TSTransform,
) {
    let dist = end_pt - start_pt;
    let steps = steps + 2;
    let pts: Vec<Pos2> = (0..=steps)
        .into_iter()
        .map(|p| {
            let t = p as f32 / steps as f32;
            let smooth_t = smoother_step(t);

            // fancy interpolations I s/like very much :)/SPENT WAY TOO LONG ON

            // Handpicked coefficients
            let k = if dist.x < 0f32 {
                -2.6f32 * dist.x
            } else {
                1.3 * dist.x
            };
            let v = Pos2 {
                x: smooth_t * (dist.x - k) + k * t + start_pt.x,
                y: smooth_t * dist.y + start_pt.y,
            };
            v
        })
        .collect();

    let path = PathShape {
        points: pts,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: PathStroke {
            width: 3f32,
            color: egui::epaint::ColorMode::Solid(Color32::WHITE),
            kind: egui::StrokeKind::Middle,
        },
    };

    let mut shape = Shape::Path(path);
    shape.transform(*view);
    lines.push(shape);
}

fn draw_single_node(painter: &Painter, shapes: &mut Vec<Shape>, node: &Node, view: TSTransform) {
    let mut r: Shape = RectShape {
        rect: Rect {
            min: node.pos,
            max: node.pos + node.prototype.size,
        },
        corner_radius: 10f32.into(),
        fill: Color32::BLACK,
        stroke: Stroke::new(3f32, Color32::WHITE),
        stroke_kind: egui::StrokeKind::Inside,
        round_to_pixels: None,
        blur_width: 0f32,
        brush: None,
    }
    .into();
    r.transform(view);

    let mut name_label: Shape = TextShape::new(
        view * (node.pos + vec2(20f32, 20f32)),
        painter.layout(
            node.prototype.name.clone(),
            FontId::proportional(14f32 * view.scaling),
            Color32::WHITE,
            node.prototype.size.x * view.scaling,
        ),
        Color32::WHITE,
    )
    .into();
    //name_label.translate(view.translation);
    shapes.push(r);
    shapes.push(name_label);

    for inp in &node.prototype.inputs {
        let mut circle: Shape = CircleShape {
            center: node.pos + inp.local_position,
            radius: 5f32,
            fill: Color32::RED,
            stroke: Stroke::NONE,
        }
        .into();
        circle.transform(view);
        shapes.push(circle);
    }
}

fn draw_node(ui: &mut egui::Ui, ui_state: &mut UIState) {
    let add_f32_prototype = NodePrototype {
        name: "Add Float".to_string(),
        inputs: vec![
            InputPort {
                local_position: egui::vec2(0f32, 50f32),
                name: "First".to_string(),
            },
            InputPort {
                local_position: egui::Vec2 { x: 0f32, y: 100f32 },
                name: "Second".to_string(),
            },
        ],
        outputs: vec![OutputPort {
            local_position: egui::vec2(100f32, 50f32),
            name: "Out".to_string(),
        }],
        size: egui::vec2(100f32, 200f32),
    };

    let size = ui.available_size();

    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click_and_drag());

    /*
    if response.secondary_clicked() {
        if let Some(p) = response.interact_pointer_pos() {
            ui_state.nodes.push(Node {
                inputs: Vec::new(),
                outputs: Vec::new(),
                pos: Vec2 { x: p.x, y: p.y },
                size: egui::Vec2 { x: 50f32, y: 50f32 },
            });
            response.mark_changed();
        }
    }*/

    // Zoom!
    // https://github.com/emilk/egui/discussions/4531
    if let (true, Some(h_pos)) = (
        response.contains_pointer(),
        ui.input(|i| i.pointer.hover_pos()),
    ) {
        let mut zoom_factor = ui.input(|i| i.zoom_delta());
        if zoom_factor != 1f32 {
            let max_scaling = 5.0f32;

            let resulting_zoom = ui_state.view.scaling * zoom_factor;

            if resulting_zoom > max_scaling {
                zoom_factor = max_scaling / ui_state.view.scaling;
            }

            let world_pos = ui_state.view.inverse() * h_pos;
            //println!("Zooming on {}", world_pos);

            // The zoom transformation happens before view because it is a world-space
            // transformation.

            ui_state.view = ui_state.view
                * TSTransform::from_translation(world_pos.to_vec2())
                * TSTransform::from_scaling(zoom_factor)
                * TSTransform::from_translation(-world_pos.to_vec2());
        }
    }

    let mut draw = DrawingState {
        lines: vec![],
        other_shapes: vec![],
    };

    ui_state.act(ui, &mut response, &mut draw);

    response.context_menu(|ui| {
        ui.label("Add Node");
        if ui.button("Add").clicked() {
            ui_state.world.create_node(
                ui_state.view.inverse() * ui.min_rect().min,
                &add_f32_prototype,
            );
        }
    });

    let painter = ui.painter_at(rect);

    for l in &ui_state.world.lines {
        let diff = (l.1 - l.0) * ui_state.view.scaling;
        let len = (diff.length() / 10f32).max(1f32).min(100f32);
        draw_line(&mut draw.lines, l.0, l.1, len as usize, &ui_state.view);
    }

    for n in &ui_state.world.nodes {
        draw_single_node(&painter, &mut draw.other_shapes, n, ui_state.view);
    }

    painter.extend(draw.lines);
    painter.extend(draw.other_shapes);
}
