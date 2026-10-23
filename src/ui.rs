pub(crate) mod panels;
pub(crate) mod theme;
pub(crate) mod views;
pub(crate) mod widgets;

/// A top-level application panel that attaches to an [`egui::Ui`]
/// (such as `egui::Panel::top`, `bottom`, `left`, or `right`).
pub(crate) trait Panel {
    fn show(&mut self, ui: &mut egui::Ui);
}

/// A content view that renders within an allocated [`egui::Ui`] region.
pub(crate) trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
