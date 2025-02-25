#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod estimate;
pub use app::TemplateApp;
pub use estimate::Estimate;
pub use estimate::EstimateApp;
pub use estimate::Task;
