use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct TopBarPanel {}

impl Panel for TopBarPanel {
    fn show(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_panel").show(ui, |_ui| {
            // The top panel is often a good place for a menu bar:
            // egui::MenuBar::new().ui(ui, |ui| {
            //     let button = ui.button("Hello World!");
            //     if button.clicked() {}
            // });
        });
    }
}
