#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod estimate;
mod task_drawer;
pub use app::TemplateApp;
pub use estimate::Estimate;
pub use estimate::EstimateApp;
pub use estimate::Task;
pub use task_drawer::draw_task_with_children;
