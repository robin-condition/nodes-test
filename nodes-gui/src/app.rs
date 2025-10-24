// https://github.com/emilk/eframe_template/blob/main/src/app.rs

use std::collections::HashMap;

use egui::{
    Align, Area, Color32, FontId, Label, Painter, Pos2, Rect, Response, Sense, Shape, Stroke,
    Widget,
    emath::TSTransform,
    epaint::{CircleShape, PathShape, PathStroke, RectShape, TextShape},
    text::LayoutJob,
    vec2,
};

pub mod storage;

use storage::*;

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
                selection: Default::default(),
            },
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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

#[derive(Clone)]
enum SourceRef {
    OutputPortRef(ID),
}

struct Connector {
    pos: egui::Vec2,
    connection: Option<SourceRef>,
}

#[derive(Clone, PartialEq, Eq)]
enum PortKindPrototype {
    Input,
    Output,
}

impl PortKindPrototype {
    fn instantiate(&self) -> PortKind {
        match self {
            PortKindPrototype::Input => PortKind::Input(None),
            PortKindPrototype::Output => PortKind::Output,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum PortKind {
    Input(Option<ID>),
    Output,
}

impl PortKind {
    pub fn is_input(&self) -> bool {
        match self {
            PortKind::Input(_) => true,
            PortKind::Output => false,
        }
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }
}

#[derive(Clone)]
pub struct Port {
    port_info: PortPrototype,
    node: ID,
    connection_kind: PortKind,
}

#[derive(Clone)]
struct PortPrototype {
    local_position: egui::Vec2,
    name: String,
    kind: PortKindPrototype,
}

// Contains all rendering information for a kind of node.
#[derive(Clone)]
pub struct NodePrototype {
    name: String,
    ports: Vec<PortPrototype>,
    size: egui::Vec2,
}

pub struct Node {
    ports: Vec<ID>,

    // In future, should be Rc or id into list of existing prototypes
    prototype: NodePrototype,

    pos: egui::Pos2,
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
                pos,
            })
            .1;

        for p in &prototype.ports {
            let new_p = self.create_port_and_link(new_obj, p);
            self.nodes.get_mut(new_obj).ports.push(new_p);
        }
    }
}

enum DrawingConnection {
    FromInput(ID),
    FromOutput(ID),
}

enum InteractingMode {
    Idle,

    DrawingConnection(DrawingConnection),

    Panning,
    Moving(Pos2, ID),
}

enum PortOrPos {
    Port(ID),
    Pos(Pos2),
}

struct UIState {
    world: NodeWorld,
    view: TSTransform,
    interacting_mode: InteractingMode,
    selection: SelectionState,
}

#[derive(Default)]
struct SelectionState {
    selected_nodes: Vec<ID>,
    hovered_port: Option<ID>,
}

struct DrawingState {
    lines: Vec<Shape>,
    other_shapes: Vec<Shape>,
}

impl UIState {
    fn selected_node(&self, pos: Pos2) -> Option<(ID, &Node)> {
        for (id, n) in self.world.nodes.with_ids() {
            let rect = Rect::from_min_size(n.pos, n.prototype.size);
            if rect.contains(pos) {
                return Some((*id, n));
            }
        }
        None
    }

    fn selected_port_pred(&self, pos: Pos2, pred: impl Fn(&Port) -> bool) -> Option<ID> {
        for (id, p) in self
            .world
            .ports
            .with_ids()
            .into_iter()
            .filter(|f| pred(f.1))
        {
            let p_pos = self.world.get_port_pos(*id); //p.pos
            let dist_square = (p_pos - pos).length_sq();
            if dist_square < 100f32 {
                return Some(*id);
            }
        }
        None
    }

    fn selected_port(&self, pos: Pos2) -> Option<ID> {
        self.selected_port_pred(pos, |_| true)
    }

    fn selected_port_of_kind(&self, pos: Pos2, kind: PortKindPrototype) -> Option<ID> {
        self.selected_port_pred(pos, |f| f.port_info.kind == kind)
    }

    pub fn act(
        &mut self,
        ui: &mut egui::Ui,
        response: &mut Response,
        drawing_state: &mut DrawingState,
    ) {
        self.selection.hovered_port = None;

        let inputs_hoverable = match &self.interacting_mode {
            InteractingMode::Idle => true,
            InteractingMode::DrawingConnection(DrawingConnection::FromOutput(_)) => true,
            _ => false,
        };

        let outputs_hoverable = match &self.interacting_mode {
            InteractingMode::Idle => true,
            InteractingMode::DrawingConnection(DrawingConnection::FromInput(_)) => true,
            _ => false,
        };

        let mouse_pos = response
            .hover_pos()
            .or(response.interact_pointer_pos())
            .map(|p| self.view.inverse() * p);
        let contains_ptr = response.contains_pointer();
        let mouse_down = response.is_pointer_button_down_on();
        let drag_started = response.drag_started();
        let dragging = response.dragged();
        let drag_stopped = response.drag_stopped();

        if let (true, Some(pos)) = (inputs_hoverable || outputs_hoverable, mouse_pos) {
            let pred: fn(&Port) -> bool = if inputs_hoverable && outputs_hoverable {
                |_| true
            } else if inputs_hoverable {
                |f| f.connection_kind.is_input()
            } else {
                |f| f.connection_kind.is_output()
            };

            self.selection.hovered_port = self.selected_port_pred(pos, pred);
        }

        let hovered_node = if contains_ptr {
            mouse_pos.map(|f| self.selected_node(f))
        } else {
            None
        };

        match &self.interacting_mode {
            InteractingMode::Idle => {
                if let Some(p) = mouse_pos {
                    if let (true, Some(port)) = (mouse_down, self.selection.hovered_port) {
                        self.interacting_mode = InteractingMode::DrawingConnection(
                            match self.world.ports.get(port).connection_kind {
                                PortKind::Input(_) => DrawingConnection::FromInput(port),
                                PortKind::Output => DrawingConnection::FromOutput(port),
                            },
                        );
                    } else if response.drag_started() {
                        if let Some(node_to_drag) = self.selected_node(p) {
                            self.interacting_mode = InteractingMode::Moving(p, node_to_drag.0);
                        } else {
                            self.interacting_mode = InteractingMode::Panning;
                        }
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
            InteractingMode::Moving(pos, node) => {
                self.selection.selected_nodes = vec![*node];
                if response.drag_stopped() {
                    self.interacting_mode = InteractingMode::Idle;
                } else if let (true, Some(p_pos)) =
                    (response.contains_pointer(), response.interact_pointer_pos())
                {
                    let world_pos = self.view.inverse() * p_pos;
                    let diff = world_pos - *pos;

                    self.world.nodes.get_mut(*node).pos += diff;
                    self.interacting_mode = InteractingMode::Moving(world_pos, *node);
                }
            }
            InteractingMode::DrawingConnection(con) => {
                if let (true, Some(pos)) = (contains_ptr, mouse_pos) {
                    let begin_port = match con {
                        DrawingConnection::FromInput(i) => *i,
                        DrawingConnection::FromOutput(i) => *i,
                    };
                    if let Some(snap_port) = self.selection.hovered_port {
                        if self.world.ports.get(snap_port).node
                            == self.world.ports.get(begin_port).node
                        {
                            self.selection.hovered_port = None;
                        }
                    }

                    let mut start_point = if let Some(end_port) = self.selection.hovered_port {
                        self.world.get_port_pos(end_port)
                    } else {
                        pos
                    };

                    let mut dest_point = self.world.get_port_pos(begin_port);

                    if let DrawingConnection::FromOutput(_) = con {
                        std::mem::swap(&mut dest_point, &mut start_point);
                    }

                    draw_line(
                        &mut drawing_state.lines,
                        start_point,
                        dest_point,
                        100usize,
                        &self.view,
                    );

                    if !mouse_down {
                        let (outp_port, inp_port) = match con {
                            DrawingConnection::FromInput(_) => {
                                (self.selection.hovered_port, Some(begin_port))
                            }
                            DrawingConnection::FromOutput(_) => {
                                (Some(begin_port), self.selection.hovered_port)
                            }
                        };

                        if let (out, Some(inp)) = (outp_port, inp_port) {
                            self.world.ports.get_mut(inp).connection_kind = PortKind::Input(out);
                        }

                        self.interacting_mode = InteractingMode::Idle;
                    }
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

fn draw_text(
    painter: &Painter,
    text: String,
    pos: Pos2,
    font_size: f32,
    halign: Align,
    valign: Align,
    view: TSTransform,
) -> TextShape {
    let mut job = LayoutJob::simple_singleline(
        text,
        FontId::proportional(font_size * view.scaling),
        Color32::WHITE,
    );
    job.halign = halign;
    let galley = painter.layout_job(job);
    let rect = galley.rect;
    TextShape::new(
        view * pos - vec2(0f32, rect.bottom() * valign.to_factor()),
        galley,
        Color32::WHITE,
    )
}

fn draw_port(
    shapes: &mut Vec<Shape>,
    painter: &Painter,
    text: String,
    pos: Pos2,
    node_side: Align,
    view: TSTransform,
    color: Color32,
) {
    let mut circle: Shape = CircleShape {
        center: pos,
        radius: 5f32,
        fill: color,
        stroke: Stroke::NONE,
    }
    .into();
    circle.transform(view);
    shapes.push(circle);

    let text_view = draw_text(
        painter,
        text,
        pos + vec2(10f32, 0f32) - node_side.to_factor() * vec2(20f32, 0f32),
        10f32,
        node_side,
        Align::Center,
        view,
    );
    shapes.push(text_view.into());
}

fn draw_single_node(
    painter: &Painter,
    shapes: &mut Vec<Shape>,
    world: &NodeWorld,
    node: &Node,
    view: TSTransform,
    select_state: &SelectionState,
) {
    let mut r: Shape = RectShape {
        rect: Rect {
            min: node.pos,
            max: node.pos + node.prototype.size,
        },
        corner_radius: 10f32.into(),
        fill: Color32::BLACK,
        stroke: Stroke::new(3f32, Color32::WHITE),
        stroke_kind: egui::StrokeKind::Middle,
        round_to_pixels: None,
        blur_width: 0f32,
        brush: None,
    }
    .into();
    r.transform(view);

    let name_label = draw_text(
        painter,
        node.prototype.name.clone(),
        node.pos + vec2(20f32, 20f32),
        14f32,
        Align::LEFT,
        Align::TOP,
        view,
    )
    .into();
    //name_label.translate(view.translation);
    shapes.push(r);
    shapes.push(name_label);

    for inp in &node.ports {
        let p = world.ports.get(*inp);
        draw_port(
            shapes,
            painter,
            p.port_info.name.clone(),
            node.pos + p.port_info.local_position,
            if p.connection_kind.is_input() {
                Align::LEFT
            } else {
                Align::RIGHT
            },
            view,
            if select_state.hovered_port == Some(*inp) {
                Color32::WHITE
            } else {
                Color32::RED
            },
        );
    }
}

fn draw_node(ui: &mut egui::Ui, ui_state: &mut UIState) {
    let add_f32_prototype = NodePrototype {
        name: "Add Float".to_string(),
        size: egui::vec2(100f32, 200f32),
        ports: vec![
            PortPrototype {
                local_position: egui::vec2(0f32, 50f32),
                name: "First".to_string(),
                kind: PortKindPrototype::Input,
            },
            PortPrototype {
                local_position: egui::Vec2 { x: 0f32, y: 100f32 },
                name: "Second".to_string(),
                kind: PortKindPrototype::Input,
            },
            PortPrototype {
                local_position: egui::vec2(100f32, 50f32),
                name: "Out".to_string(),
                kind: PortKindPrototype::Output,
            },
        ],
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

    /*
    for l in &ui_state.world.lines {
        let diff = (l.1 - l.0) * ui_state.view.scaling;
        let len = (diff.length() / 10f32).max(1f32).min(100f32);
        draw_line(&mut draw.lines, l.0, l.1, len as usize, &ui_state.view);
    }
    */

    for p in &ui_state.world.ports {
        match &p.connection_kind {
            PortKind::Input(Some(outp_id)) => {
                let l = (
                    ui_state.world.get_port_pos(*outp_id),
                    ui_state.world.get_port_pos_from_ref(p),
                );

                let diff = (l.1 - l.0) * ui_state.view.scaling;
                let len = (diff.length() / 10f32).max(1f32).min(100f32);
                draw_line(&mut draw.lines, l.0, l.1, len as usize, &ui_state.view);
            }
            _ => {}
        }
    }

    for n in &ui_state.world.nodes {
        draw_single_node(
            &painter,
            &mut draw.other_shapes,
            &ui_state.world,
            n,
            ui_state.view,
            &ui_state.selection,
        );
    }

    painter.extend(draw.other_shapes);
    painter.extend(draw.lines);
}
