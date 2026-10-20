pub(crate) mod components;
pub(crate) mod layouts;
pub(crate) mod panels;
pub(crate) mod views;
pub(crate) mod widgets;

pub(crate) trait Updatable {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // do nothing
    }
}
