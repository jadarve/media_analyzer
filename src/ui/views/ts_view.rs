use crate::loader::TsPacket;
use crate::ui::widgets::TsPacketWidget;
use crate::ui::View;

#[derive(Default)]
pub(crate) struct TsView {
    selected_packet: Option<TsPacket>,
}

impl TsView {
    pub(crate) fn set_packet(&mut self, packet: Option<TsPacket>) {
        self.selected_packet = packet;
    }
}

impl View for TsView {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(packet) = &self.selected_packet {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add(TsPacketWidget::new(packet));
                });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new(
                        "Open a Transport Stream (.ts) file from the left toolbar to inspect packets.",
                    )
                    .weak()
                    .size(16.0),
                );
            });
        }
    }
}
