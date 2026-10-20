use crate::mediadoc::ui::Updatable;

#[derive(Default)]
pub(crate) struct TsView {
    // any state?
}

impl Updatable for TsView {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            // egui::menu::bar(ui, |ui| {
            //     let button = ui.button("Hello World!");
            //     if button.clicked() {}
            // });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Hello World!");
                ui.add(egui::Separator::default().vertical());
            });

        egui::SidePanel::left("left_tool_bar")
            .resizable(false)
            .exact_width(60.0)
            .show(ctx, |ui| {
                ui.heading("Tools!");
                // ui.add(egui::Separator::default().horizontal());
            });

        egui::SidePanel::left("left_side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Left side panel!");
                ui.add(egui::Separator::default().horizontal());
            });

        egui::SidePanel::right("right_side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Right side panel!");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Central Panel!");
        });
    }
}
