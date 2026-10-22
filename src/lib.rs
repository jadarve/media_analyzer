#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MediadocApp;

pub(crate) mod loader;
pub(crate) mod ui;
