use egui::{Pos2, Rect};

use crate::{
    task_drawer::{draw_task, paint_rectangle, TaskPosition, RADII},
    EstimateApp,
};

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

    input_field_state: InputFieldAction,
    input_field_text: String,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
enum InputFieldAction {
    Hide,
    CreateTask,
    EditEstimate,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            estimate_app: EstimateApp::new(),
            input_field_state: InputFieldAction::Hide,
            selected_task_id: None,
            input_field_text: "".to_owned(),
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
        if ctx.input(|i| i.key_pressed(egui::Key::N))
            && self.input_field_state == InputFieldAction::Hide
        {
            self.input_field_state = InputFieldAction::CreateTask;
            self.input_field_text = "".to_owned();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape))
            && self.input_field_state == InputFieldAction::Hide
        {
            self.selected_task_id = None;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape))
            && self.input_field_state != InputFieldAction::Hide
        {
            self.input_field_state = InputFieldAction::Hide;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::A))
            && self.input_field_state == InputFieldAction::Hide
        {
            let id = self
                .estimate_app
                .previous_task_id(self.selected_task_id.as_deref());
            self.selected_task_id = id;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::D))
            && self.input_field_state == InputFieldAction::Hide
        {
            let id = self
                .estimate_app
                .next_task_id(self.selected_task_id.as_deref());
            self.selected_task_id = id;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            println!("Enter pressed, current state: {:?}", self.input_field_state);
            match self.input_field_state {
                InputFieldAction::Hide => {
                    println!("Enter pressed, showing edit task estimate");
                    if let Some(id) = &self.selected_task_id {
                        let current_task = self.estimate_app.find_task(id);
                        if let Some(task) = current_task {
                            println!("Task found: {:?}", task);
                            self.input_field_text = task.estimate.to_string();
                            self.input_field_state = InputFieldAction::EditEstimate;
                        }
                    }
                }
                InputFieldAction::CreateTask => {
                    println!("Enter pressed, creating task and hiding input field");
                    self.input_field_text = self.input_field_text.clone();
                    self.input_field_state = InputFieldAction::Hide;

                    if let Some(id) = &self.selected_task_id {
                        let task = self.estimate_app.find_mut_task(id.as_str());
                        if let Some(task) = task {
                            let task_id = task.add_child_task(&self.input_field_text, 0);
                            self.selected_task_id = Some(task_id);
                        }
                    } else {
                        let task_id = self.estimate_app.add_task(&self.input_field_text);
                        self.selected_task_id = Some(task_id);
                    }
                }
                InputFieldAction::EditEstimate => {
                    println!("Enter pressed, saving estimate and hiding input field");
                    if let Some(id) = &self.selected_task_id {
                        let current_task = self.estimate_app.find_mut_task(id);
                        if let Some(task) = current_task {
                            task.estimate = self.input_field_text.parse().unwrap_or(0);
                            println!("Task saved: {:?}", task);
                        }
                    }
                    self.input_field_state = InputFieldAction::Hide;
                }
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
                    ui.button(format!("{:?}", self.input_field_state));
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::click(),
            );

            if response.clicked() {
                self.selected_task_id = None;
            }

            let rect = Rect::from_center_size(response.rect.center(), RADII * 2.0);
            paint_rectangle(&painter, rect, false, "Root".to_string(), None, 0);

            let mut placed_positions: Vec<Pos2> = Vec::new();
            // Ensure that you get mutable access to tasks (assuming get_tasks_mut() exists).
            let tasks = self.estimate_app.get_tasks_mut();
            let num_tasks = tasks.len();
            if num_tasks > 0 {
                for (index, task) in tasks.iter().enumerate() {
                    //
                    let placed_pos = draw_task(
                        &painter,
                        task,
                        response.rect.width().min(response.rect.height()) * 0.3,
                        &placed_positions,
                        self.selected_task_id.as_deref(),
                        TaskPosition::new(
                            task.id.clone(),
                            response.rect.center(),
                            rect,
                            false,
                            index,
                            num_tasks,
                            0,
                        ),
                    );
                    placed_positions.push(placed_pos);
                }
            }
        });

        // Optionally, if you want to draw the input field when show_input_field is true:
        if self.input_field_state != InputFieldAction::Hide {
            egui::Window::new("New Task").show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.input_field_text)
                    .request_focus();
                if ui.button("Submit").clicked() {
                    self.input_field_state = InputFieldAction::Hide;
                }
            });
        }
    }
}
