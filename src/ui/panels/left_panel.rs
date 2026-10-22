use std::rc::Rc;

use crate::loader::TsContainer;
use crate::ui::Panel;

#[derive(Default)]
pub(crate) struct LeftSidePanel {
    container: Option<Rc<TsContainer>>,
    selected_packet: Option<usize>,
    selection_changed: bool,
}

impl LeftSidePanel {
    pub(crate) fn set_container(&mut self, container: Rc<TsContainer>) {
        let first_packet = if container.is_empty() { None } else { Some(0) };
        self.container = Some(container);
        self.selected_packet = first_packet;
        self.selection_changed = true;
    }

    pub(crate) fn selected_packet(&self) -> Option<usize> {
        self.selected_packet
    }

    pub(crate) fn take_selection_changed(&mut self) -> bool {
        std::mem::take(&mut self.selection_changed)
    }
}

impl Panel for LeftSidePanel {
    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_side_panel")
            .resizable(true)
            .default_width(250.0)
            .min_width(180.0)
            .show(ctx, |ui| {
                ui.heading("TS Packets");
                if let Some(container) = &self.container {
                    ui.label(
                        egui::RichText::new(format!(
                            "{} ({} packets)",
                            container.name(),
                            container.len()
                        ))
                        .small()
                        .weak(),
                    );
                } else {
                    ui.label(egui::RichText::new("No file loaded").small().weak());
                }
                ui.add(egui::Separator::default().horizontal());

                if let Some(container) = &self.container {
                    let total_packets = container.len();
                    if total_packets == 0 {
                        ui.label("No complete 188-byte TS packets found in file.");
                        return;
                    }

                    let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 4.0;
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show_rows(ui, row_height, total_packets, |ui, row_range| {
                            for idx in row_range {
                                let pid = container.pid_at(idx).unwrap_or(0);
                                let is_selected = self.selected_packet == Some(idx);
                                let label_text = egui::RichText::new(format!(
                                    "Packet #{idx} — PID 0x{pid:04X} ({pid})"
                                ))
                                .monospace();

                                if ui.selectable_label(is_selected, label_text).clicked()
                                    && self.selected_packet != Some(idx)
                                {
                                    self.selected_packet = Some(idx);
                                    self.selection_changed = true;
                                }
                            }
                        });
                }
            });
    }
}
