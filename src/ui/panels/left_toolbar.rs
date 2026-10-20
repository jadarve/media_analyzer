use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct LeftToolbarPanel {}

impl Panel for LeftToolbarPanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_tool_bar")
            .resizable(false)
            .exact_width(60.0)
            .show(ctx, |ui| {
                ui.heading("Tools!");
                // ui.add(egui::Separator::default().horizontal());
            });
    }
}

