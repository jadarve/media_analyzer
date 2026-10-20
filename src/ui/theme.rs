/// Configures the global [`egui::Visuals`] and styling for the application.
pub(crate) fn apply(ctx: &egui::Context) {
    let visuals = egui::Visuals::light();
    ctx.set_visuals(visuals);
}
