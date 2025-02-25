use egui::Pos2;

use crate::EstimateApp;
use crate::Task;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    estimate_app: EstimateApp,

    selected_task_id: Option<String>,

    show_input_field: bool,
    input_field_text: String,
    input_field_value: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            estimate_app: EstimateApp::new(),
            show_input_field: false,
            selected_task_id: None,
            input_field_text: "Type something here...".to_owned(),
            input_field_value: "".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Listen for the N key press to show the input field.
        if ctx.input(|i| i.key_pressed(egui::Key::N)) && !self.show_input_field {
            self.show_input_field = true;
            self.input_field_text = "".to_owned();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && self.show_input_field {
            self.input_field_value = self.input_field_text.clone();
            self.show_input_field = false;

            if !self.selected_task_id.is_none() {
                if let Some(selected_task_id) = &self.selected_task_id {
                    let selected_task = self
                        .estimate_app
                        .tasks
                        .iter_mut()
                        .find(|task| task.id == *selected_task_id);
                    if let Some(selected_task) = selected_task {
                        selected_task.add_child_task(&self.input_field_value, 0);
                    } else {
                        self.estimate_app.add_task(&self.input_field_text);
                    }
                }
            } else {
                self.estimate_app.add_task(&self.input_field_text);
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button(format!("File-{:?}", self.selected_task_id), |ui| {
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
            let (response, _painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::click(),
            );

            if response.clicked() {
                self.selected_task_id = None;
            }

            let mut placed_positions: Vec<Pos2> = Vec::new();
            // Ensure that you get mutable access to tasks (assuming get_tasks_mut() exists).
            let tasks = self.estimate_app.get_tasks_mut();
            let num_tasks = tasks.len();
            if num_tasks > 0 {
                let center = response.rect.center();
                let radius = response.rect.width().min(response.rect.height()) * 0.3;
                for (i, task) in tasks.iter_mut().enumerate() {
                    let angle = i as f32 / num_tasks as f32 * std::f32::consts::TAU;
                    let pos = egui::pos2(
                        center.x + radius * angle.cos(),
                        center.y + radius * angle.sin(),
                    );
                    let radii = egui::vec2(75.0, 25.0);

                    let mut current_radius = radius;
                    let mut adjusted_pos = pos;
                    let safe_distance = (75.0_f32.powi(2) + 25.0_f32.powi(2)).sqrt() * 2.0;
                    while placed_positions
                        .iter()
                        .any(|&p| p.distance(adjusted_pos) < safe_distance)
                    {
                        current_radius += 10.0;
                        adjusted_pos = egui::pos2(
                            center.x + current_radius * angle.cos(),
                            center.y + current_radius * angle.sin(),
                        );
                    }
                    placed_positions.push(adjusted_pos);
                    let pos = adjusted_pos;
                    let rect = egui::Rect::from_center_size(pos, radii * 2.0);

                    // Handle click event: toggle the task.selected property.
                    let response =
                        ui.interact(rect, egui::Id::new(task.name.clone()), egui::Sense::click());
                    if response.clicked() {
                        self.selected_task_id = Some(task.id.clone());
                    }

                    let painter = ui.painter();

                    // Draw the parent rectangle.
                    painter.rect(
                        rect,
                        0.0,
                        egui::Color32::WHITE,
                        egui::Stroke::new(2.0, egui::Color32::BLACK),
                    );
                    // If the parent task is selected, add a blue outline.
                    if self.selected_task_id == Some(task.id.clone()) {
                        painter.rect(
                            rect,
                            0.0,
                            egui::Color32::TRANSPARENT,
                            egui::Stroke::new(5.0, egui::Color32::BLUE),
                        );
                    }
                    // Draw the task name.
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", task.name),
                        egui::FontId::proportional(16.0),
                        egui::Color32::BLACK,
                    );
                    // Draw a line from the parent's rectangle center to the UI center.
                    let ui_center = painter.clip_rect().center();
                    let from_y = if rect.top() < ui_center.y {
                        rect.bottom()
                    } else {
                        rect.top()
                    };
                    painter.line_segment(
                        [egui::pos2(rect.center().x, from_y), ui_center],
                        egui::Stroke::new(1.5, egui::Color32::DARK_GRAY),
                    );

                    // --- NEW SUB-LOOP: Draw children of the task ---
                    // Position children in a half circle away from the UI center.
                    if !task.children.is_empty() {
                        let num_children = task.children.len();
                        // Compute the base angle so that the half-circle faces away from the UI center.
                        let base_angle = (pos - ui_center).angle();
                        let arc_span = std::f32::consts::PI; // 180°
                        let distance_from_parent = 200.0; // Adjust as needed for spacing.
                        for (j, child) in task.children.iter_mut().enumerate() {
                            // Calculate the angle for each child along the half circle.
                            let fraction = if num_children > 1 {
                                j as f32 / (num_children - 1) as f32
                            } else {
                                0.5
                            };
                            let child_angle = base_angle - (arc_span / 2.0) + fraction * arc_span;
                            // Position the child at an offset from the parent's center.
                            let child_pos = egui::pos2(
                                pos.x + distance_from_parent * child_angle.cos(),
                                pos.y + distance_from_parent * child_angle.sin(),
                            );
                            let child_rect = egui::Rect::from_center_size(child_pos, radii * 2.0);

                            // Optionally add a click handler for children (similar to parent tasks):
                            let child_response = ui.interact(
                                child_rect,
                                egui::Id::new(child.name.clone()),
                                egui::Sense::click(),
                            );
                            if child_response.clicked() {
                                self.selected_task_id = Some(child.id.clone());
                            }

                            // Draw the child rectangle.
                            painter.rect(
                                child_rect,
                                0.0,
                                egui::Color32::WHITE,
                                egui::Stroke::new(2.0, egui::Color32::BLACK),
                            );
                            // Draw blue outline if the child is selected.
                            if self.selected_task_id == Some(child.id.clone()) {
                                painter.rect(
                                    child_rect,
                                    0.0,
                                    egui::Color32::TRANSPARENT,
                                    egui::Stroke::new(5.0, egui::Color32::BLUE),
                                );
                            }
                            // Draw the child task name.
                            painter.text(
                                child_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                format!("{}", child.name),
                                egui::FontId::proportional(16.0),
                                egui::Color32::BLACK,
                            );
                            // Draw a line from the child rectangle center to the parent's rectangle center.
                            painter.line_segment(
                                [child_rect.center(), rect.center()],
                                egui::Stroke::new(1.5, egui::Color32::DARK_GRAY),
                            );
                        }
                    }
                }
            }
        });

        // Optionally, if you want to draw the input field when show_input_field is true:
        if self.show_input_field {
            egui::Window::new("New Task").show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.input_field_text)
                    .request_focus();
                if ui.button("Submit").clicked() {
                    self.input_field_value = self.input_field_text.clone();
                    self.show_input_field = false;
                }
            });
        }
    }
}
