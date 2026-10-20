use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct BottomPanel {}

impl Panel for BottomPanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Hello World!");
                ui.add(egui::Separator::default().vertical());
            });
    }
}

