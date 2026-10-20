use crate::ui::View;

#[derive(Default)]
pub(crate) struct TsView {
    // any state?
}

impl View for TsView {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Central Panel!");
    }
}

