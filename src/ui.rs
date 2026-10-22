pub(crate) mod panels;
pub(crate) mod theme;
pub(crate) mod views;
pub(crate) mod widgets;

/// A top-level application panel that attaches directly to [`egui::Context`]
/// (such as `TopBottomPanel` or `SidePanel`).
pub(crate) trait Panel {
    fn show(&mut self, ctx: &egui::Context);
}

/// A content view that renders within an allocated [`egui::Ui`] region.
pub(crate) trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
