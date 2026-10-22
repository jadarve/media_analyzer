use crate::ui::Panel;

pub(crate) struct BottomPanel {
    status: String,
    is_error: bool,
}

impl Default for BottomPanel {
    fn default() -> Self {
        Self {
            status: "Ready".to_owned(),
            is_error: false,
        }
    }
}

impl BottomPanel {
    pub(crate) fn set_status(&mut self, status: impl Into<String>) {
        self.status = status.into();
        self.is_error = false;
    }

    pub(crate) fn set_error(&mut self, error: impl Into<String>) {
        self.status = error.into();
        self.is_error = true;
    }
}

impl Panel for BottomPanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let text = egui::RichText::new(&self.status);
                    if self.is_error {
                        ui.label(text.color(egui::Color32::from_rgb(200, 40, 40)));
                    } else {
                        ui.label(text.weak());
                    }
                });
            });
    }
}
