use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct LeftSidePanel {}

impl Panel for LeftSidePanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Left side panel!");
                ui.add(egui::Separator::default().horizontal());
            });
    }
}

