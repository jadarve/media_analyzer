use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct RightSidePanel {}

impl Panel for RightSidePanel {
    fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::right("right_side_panel")
            .resizable(true)
            .show(ui, |ui| {
                ui.heading("Right side panel!");
            });
    }
}
