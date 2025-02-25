use egui::epaint::CircleShape;
use egui::epaint::Shape;
use egui::Context;
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

    show_input_field: bool,
    input_field_text: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            estimate_app: EstimateApp::new(),
            show_input_field: false,
            input_field_text: "Type something here...".to_owned(),
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        //ctx.set_fonts(font_definitions);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
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
            let (response, _painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::hover(),
            );

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
                        task.selected = !task.selected;
                    }

                    let painter = ui.painter();

                    // Draw the rectangle with a black stroke.
                    painter.rect(
                        rect,
                        0.0,
                        egui::Color32::WHITE,
                        egui::Stroke::new(2.0, egui::Color32::BLACK),
                    );

                    // If the task is selected, add a blue outline.
                    if task.selected {
                        painter.rect(
                            rect,
                            0.0,
                            egui::Color32::TRANSPARENT,
                            egui::Stroke::new(5.0, egui::Color32::BLUE),
                        );
                    }

                    // Draw the task name centered inside the rectangle.
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{}", task.name),
                        egui::FontId::proportional(16.0),
                        egui::Color32::BLACK,
                    );

                    // Draw a line from the rectangle center to the center of the UI.
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
                }
            }
        });
        /*
           egui::CentralPanel::default().show(ctx, |ui| {
               let (response, painter) =
                   ui.allocate_painter(egui::Vec2::new(200.0, 200.0), egui::Sense::hover());

               let radius = 20.0;
               let spacing = 50.0;
               let center_x = response.rect.center().x;
               let center_y = response.rect.center().y;

               for i in 0..3 {
                   let x = center_x + (i as f32 - 1.0) * spacing;
                   let y = center_y;
                   painter.add(Shape::Circle(CircleShape {
                       center: egui::Pos2::new(x, y),
                       radius,
                       fill: egui::Color32::WHITE,
                       stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                   }));
               }
           });
        */
    }
}

impl TemplateApp {
    /// Draws a rectangle with the task name centered inside,
    /// draws a blue outline if the task is selected,
    /// toggles task.selected on click,
    /// and draws a line from the rectangle center to the UI center.
    ///
    /// - ui: The egui UI handle used for drawing and interaction.
    /// - task: The mutable task containing the name and selection state.
    /// - center: The center point of the rectangle.
    /// - radii: Half of the rectangle's width and height.
    pub fn draw_task(
        &mut self,
        ui: &mut egui::Ui,
        task: &mut Task,
        center: Pos2,
        radii: egui::Vec2,
    ) {
        // Create a rectangle centered on `center` with size (radii * 2).
        let rect = egui::Rect::from_center_size(center, radii * 2.0);

        // Handle click event: toggle the task.selected property.
        let response = ui.interact(rect, egui::Id::new(task.name.clone()), egui::Sense::click());
        if response.clicked() {
            task.selected = !task.selected;
        }

        let painter = ui.painter();

        // Draw the rectangle with a black stroke.
        painter.rect(
            rect,
            0.0,
            egui::Color32::WHITE,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
        );

        // If the task is selected, add a blue outline.
        if task.selected {
            painter.rect(
                rect,
                0.0,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(5.0, egui::Color32::BLUE),
            );
        }

        // Draw the task name centered inside the rectangle.
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}", task.name),
            egui::FontId::proportional(16.0),
            egui::Color32::BLACK,
        );

        // Draw a line from the rectangle center to the center of the UI.
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
    }
}
