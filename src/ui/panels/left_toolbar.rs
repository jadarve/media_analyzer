use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct LeftToolbarPanel {
    open_file_requested: bool,
}

impl LeftToolbarPanel {
    pub(crate) fn take_open_file_request(&mut self) -> bool {
        std::mem::take(&mut self.open_file_requested)
    }
}

impl Panel for LeftToolbarPanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_tool_bar")
            .resizable(false)
            .exact_width(48.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    let open_btn = ui
                        .add_sized(
                            [36.0, 36.0],
                            egui::Button::new(egui::RichText::new("📂").size(20.0)),
                        )
                        .on_hover_text("Open Transport Stream (.ts) file");

                    if open_btn.clicked() {
                        self.open_file_requested = true;
                    }
                });
            });
    }
}
