#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod estimate_app;
mod task;
mod task_drawer;
pub use app::TemplateApp;
pub use estimate_app::EstimateApp;
pub use task::Task;
