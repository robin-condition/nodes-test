// https://github.com/emilk/eframe_template/blob/main/src/app.rs

use egui::{
    emath::TSTransform, epaint::{CornerRadiusF32, PathShape, PathStroke, RectShape, TextShape}, Color32, CornerRadius, Pos2, Rect, Sense, Shape, Stroke, Vec2
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
                nodes: Vec::new(),
                draw_start: None,
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

struct NodeId(usize);

enum SourceRef {
    Connector(usize),
    OutputPortRef(NodeId, usize),
}

struct Connector {
    pos: egui::Vec2,
    connection: Option<SourceRef>,
}

struct InputPort {
    connection: Option<SourceRef>,
    local_position: egui::Vec2,
    name: String,
}

struct OutputPort {
    local_position: egui::Vec2,
    name: String,
}

struct Node {
    inputs: Vec<InputPort>,
    outputs: Vec<OutputPort>,
    pos: egui::Vec2,
    size: egui::Vec2,
}

struct UIState {
    nodes: Vec<Node>,
    draw_start: Option<Pos2>,
    view: TSTransform,
}

fn add_node(ui_state: &mut UIState) {

}

// Derived myself :)
// From:
// f(0)=0, f(1)=1
// f'(0)=f'(1)=0
// f''(0)=f''(1)=0
// Then I googled it and found Knuth proposed it apparently
fn smoother_step(t: f32) -> f32 {
    ((6f32 * t - 15f32) * t + 10f32) * t.powi(3)
}

fn draw_line(lines: &mut Vec<Shape>, start_pt: Pos2, end_pt: Pos2, steps: usize) {
    let dist = end_pt - start_pt;
    let steps = steps + 2;
    let pts: Vec<Pos2> = (0..=steps).into_iter().map(|p| {
        let t = p as f32 / steps as f32;
        let smooth_t = smoother_step(t);

        // fancy interpolations I s/like very much :)/SPENT WAY TOO LONG ON

        // for x:
        let k = if dist.x < 0f32 {
            -2.6f32 * dist.x
        } else {
            1.3 * dist.x
        };
        let v = Pos2 {
            x: smooth_t * (dist.x - k) + k * t + start_pt.x,
            y: smooth_t * dist.y + start_pt.y
        };
        v
    }).collect();

    let path = 
    PathShape {
        points: pts,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: PathStroke {
            width: 3f32,
            color: egui::epaint::ColorMode::Solid(Color32::WHITE),
            kind: egui::StrokeKind::Middle,
        },
    };

    lines.push(Shape::Path(path));
}

fn draw_single_node(shapes: &mut Vec<Shape>, node: &Node, view: TSTransform) {

        
}

fn draw_node(ui: &mut egui::Ui, ui_state: &mut UIState) {
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
    if response.drag_started() {
        if let Some(p) = response.interact_pointer_pos() {
            let worldspace = ui_state.view.inverse() * p;
            ui_state.draw_start = Some(worldspace);
            //println!("Drag Started: {worldspace}");
        }
    }

    

    if response.drag_stopped() {
        ui_state.draw_start = None;
        //println!("Drag stopped!");
    }

    response.context_menu(|ui| {
        ui.label("Add Node");
        if ui.button("Constant").clicked() {
            add_node(ui_state);
        }
    });

    let painter = ui.painter_at(rect);

    let mut v = Vec::new();

    if response.dragged() {
        if let (Some(start_pos), Some(end_ui_pos)) = (ui_state.draw_start, response.interact_pointer_pos()) {
            let end_pos = ui_state.view.inverse() * end_ui_pos;

            //println!("Drawing from {} to {}", start_pos, end_pos);

        draw_line(&mut v, start_pos, end_pos, 100usize);
        }
    }
    
    painter.extend(v);

    for n in &ui_state.nodes {
        let r: Shape = RectShape {
            rect: Rect {
                min: Pos2 {
                    x: n.pos.x,
                    y: n.pos.y,
                },
                max: Pos2 {
                    x: n.pos.x + n.size.x,
                    y: n.pos.y + n.size.y,
                },
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
        painter.add(r);
    }
}
