#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod task;
mod task_drawer;
pub use app::TemplateApp;
pub use task::EstimateApp;
pub use task::Task;
pub use task_drawer::draw_task_with_children;
