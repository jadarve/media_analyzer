#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MediaAnalyzerApp;

pub(crate) mod loader;
pub(crate) mod ui;
