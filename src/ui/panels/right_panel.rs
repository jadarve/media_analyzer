use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct RightSidePanel {}

impl Panel for RightSidePanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Right side panel!");
            });
    }
}
