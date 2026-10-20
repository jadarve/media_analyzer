use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct TopBarPanel {}

impl Panel for TopBarPanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |_ui| {
            // The top panel is often a good place for a menu bar:
            // egui::menu::bar(ui, |ui| {
            //     let button = ui.button("Hello World!");
            //     if button.clicked() {}
            // });
        });
    }
}

