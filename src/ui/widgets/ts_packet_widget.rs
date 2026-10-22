use egui::{Color32, RichText, Ui, Widget};
use lluvia_media::containers::ts::{AdaptationFieldControl, TsError};

use crate::loader::TsPacket;

const HEADER_COLOR: Color32 = Color32::from_rgb(25, 103, 210);
const ADAPTATION_COLOR: Color32 = Color32::from_rgb(194, 100, 1);
const PAYLOAD_COLOR: Color32 = Color32::from_rgb(19, 115, 51);
const VALID_COLOR: Color32 = Color32::from_rgb(19, 115, 51);
const INVALID_COLOR: Color32 = Color32::from_rgb(200, 40, 40);

/// Widget that renders the parsed fields and color-coded hexadecimal dump of an MPEG-TS packet.
pub(crate) struct TsPacketWidget<'a> {
    packet: &'a TsPacket,
}

impl<'a> TsPacketWidget<'a> {
    pub(crate) fn new(packet: &'a TsPacket) -> Self {
        Self { packet }
    }

    fn render_summary_bar(&self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.heading(format!("Packet #{}", self.packet.index));
            ui.separator();
            ui.label(
                RichText::new(format!(
                    "Stream Offset: 0x{:08X} ({} B)",
                    self.packet.stream_offset(),
                    self.packet.stream_offset()
                ))
                .monospace(),
            );
            ui.separator();
            match self.packet.view.is_valid() {
                Ok(()) => {
                    ui.label(
                        RichText::new("✔ Valid TS Packet")
                            .color(VALID_COLOR)
                            .strong(),
                    );
                }
                Err(err) => {
                    ui.label(
                        RichText::new(format!("⚠ Invalid Packet: {err}"))
                            .color(INVALID_COLOR)
                            .strong(),
                    );
                }
            }
        });
    }

    fn render_header_fields(&self, ui: &mut Ui) {
        let view = &self.packet.view;
        egui::CollapsingHeader::new(RichText::new("Transport Stream Header (4 bytes)").strong())
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("ts_packet_header_grid")
                    .num_columns(2)
                    .striped(true)
                    .spacing([24.0, 6.0])
                    .show(ui, |ui| {
                        field_row(ui, "sync_byte", format!("0x{:02X}", view.sync_byte()));
                        field_row(
                            ui,
                            "transport_error_indicator",
                            format_bool_flag(view.transport_error_indicator()),
                        );
                        field_row(
                            ui,
                            "payload_unit_start_indicator",
                            format_bool_flag(view.payload_unit_start_indicator()),
                        );
                        field_row(
                            ui,
                            "transport_priority",
                            format_bool_flag(view.transport_priority()),
                        );

                        let pid = view.pid();
                        let pid_desc = well_known_pid_name(pid);
                        let pid_text = if let Some(desc) = pid_desc {
                            format!("0x{pid:04X} ({pid}) — {desc}")
                        } else {
                            format!("0x{pid:04X} ({pid})")
                        };
                        field_row(ui, "pid", pid_text);

                        field_row(
                            ui,
                            "transport_scrambling_control",
                            format!("{:?}", view.transport_scrambling_control()),
                        );
                        field_row(
                            ui,
                            "adaptation_field_control",
                            format!("{:?}", view.adaptation_field_control()),
                        );
                        field_row(
                            ui,
                            "continuity_counter",
                            format!("{}", view.continuity_counter()),
                        );
                    });
            });
    }

    fn render_adaptation_field(&self, ui: &mut Ui) {
        let afc = self.packet.view.adaptation_field_control();
        if !matches!(
            afc,
            AdaptationFieldControl::AdaptationFieldOnly
                | AdaptationFieldControl::AdaptationFieldAndPayload
        ) {
            return;
        }

        egui::CollapsingHeader::new(RichText::new("Adaptation Field").strong())
            .default_open(true)
            .show(ui, |ui| match self.packet.view.adaptation_field() {
                Ok(af) => {
                    egui::Grid::new("ts_packet_af_grid")
                        .num_columns(2)
                        .striped(true)
                        .spacing([24.0, 6.0])
                        .show(ui, |ui| {
                            let len = af.length();
                            field_row(ui, "adaptation_field_length", format!("{len} bytes"));

                            if len > 0 {
                                result_bool_row(
                                    ui,
                                    "discontinuity_indicator",
                                    af.discontinuity_indicator(),
                                );
                                result_bool_row(
                                    ui,
                                    "random_access_indicator",
                                    af.random_access_indicator(),
                                );
                                result_bool_row(
                                    ui,
                                    "elementary_stream_priority_indicator",
                                    af.elementary_stream_priority_indicator(),
                                );
                                result_bool_row(ui, "pcr_flag", af.pcr_flag());
                                result_bool_row(ui, "opcr_flag", af.opcr_flag());
                                result_bool_row(
                                    ui,
                                    "splicing_point_flag",
                                    af.splicing_point_flag(),
                                );
                                result_bool_row(
                                    ui,
                                    "transport_private_data_flag",
                                    af.transport_private_data_flag(),
                                );
                                result_bool_row(
                                    ui,
                                    "adaptation_field_extension_flag",
                                    af.adaptation_field_extension_flag(),
                                );
                            }
                        });
                }
                Err(err) => {
                    ui.label(RichText::new(format!("Error: {err}")).color(INVALID_COLOR));
                }
            });
    }

    fn render_payload_section(&self, ui: &mut Ui) {
        let afc = self.packet.view.adaptation_field_control();
        if !matches!(
            afc,
            AdaptationFieldControl::PayloadOnly | AdaptationFieldControl::AdaptationFieldAndPayload
        ) {
            return;
        }

        egui::CollapsingHeader::new(RichText::new("Payload").strong())
            .default_open(true)
            .show(ui, |ui| match self.packet.view.payload() {
                Ok(payload) => {
                    egui::Grid::new("ts_packet_payload_grid")
                        .num_columns(2)
                        .striped(true)
                        .spacing([24.0, 6.0])
                        .show(ui, |ui| {
                            field_row(ui, "payload_length", format!("{} bytes", payload.length()));
                        });
                }
                Err(err) => {
                    ui.label(RichText::new(format!("Error: {err}")).color(INVALID_COLOR));
                }
            });
    }

    fn render_hex_view(&self, ui: &mut Ui) {
        let raw = &self.packet.raw;
        let header_range = self.packet.header_range();
        let af_range = self.packet.adaptation_field_range();
        let payload_range = self.packet.payload_range();

        egui::CollapsingHeader::new(
            RichText::new(format!("Hexadecimal Content ({} bytes)", raw.len())).strong(),
        )
        .default_open(true)
        .show(ui, |ui| {
            // Color legend
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("■ Header (4 B)").color(HEADER_COLOR).strong());
                ui.add_space(12.0);
                if let Some(r) = &af_range {
                    ui.label(
                        RichText::new(format!("■ Adaptation Field ({} B)", r.len()))
                            .color(ADAPTATION_COLOR)
                            .strong(),
                    );
                    ui.add_space(12.0);
                }
                if let Some(r) = &payload_range {
                    ui.label(
                        RichText::new(format!("■ Payload ({} B)", r.len()))
                            .color(PAYLOAD_COLOR)
                            .strong(),
                    );
                }
            });

            ui.add_space(4.0);

            egui::Frame::canvas(ui.style())
                .inner_margin(egui::Margin::same(8.0))
                .show(ui, |ui| {
                    egui::Grid::new("ts_packet_hex_dump_grid")
                        .num_columns(3)
                        .striped(true)
                        .spacing([16.0, 4.0])
                        .show(ui, |ui| {
                            // Header row
                            ui.label(RichText::new("Offset").monospace().weak());
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 4.0;
                                for col_idx in 0..16 {
                                    if col_idx == 8 {
                                        ui.add_space(6.0);
                                    }
                                    ui.label(
                                        RichText::new(format!("{col_idx:02X}"))
                                            .monospace()
                                            .weak(),
                                    );
                                }
                            });
                            ui.label(RichText::new("ASCII").monospace().weak());
                            ui.end_row();

                            for (row_idx, chunk) in raw.chunks(16).enumerate() {
                                let row_offset = row_idx * 16;
                                ui.label(
                                    RichText::new(format!("{row_offset:04X}"))
                                        .monospace()
                                        .weak(),
                                );

                                // Hex bytes column
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 4.0;
                                    for col_idx in 0..16 {
                                        if col_idx == 8 {
                                            ui.add_space(6.0);
                                        }
                                        if let Some(&byte) = chunk.get(col_idx) {
                                            let byte_offset = row_offset + col_idx;
                                            let (color, region_name) = byte_region_style(
                                                byte_offset,
                                                &header_range,
                                                af_range.as_ref(),
                                                payload_range.as_ref(),
                                            );
                                            ui.label(
                                                RichText::new(format!("{byte:02X}"))
                                                    .monospace()
                                                    .color(color),
                                            )
                                            .on_hover_text(format!(
                                                "Byte 0x{byte_offset:02X} ({byte_offset}) — {region_name}\nDec: {byte} | Bin: 0b{byte:08b}"
                                            ));
                                        } else {
                                            ui.label(RichText::new("  ").monospace());
                                        }
                                    }
                                });

                                // ASCII column
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    for (col_idx, &byte) in chunk.iter().enumerate() {
                                        let byte_offset = row_offset + col_idx;
                                        let (color, _) = byte_region_style(
                                            byte_offset,
                                            &header_range,
                                            af_range.as_ref(),
                                            payload_range.as_ref(),
                                        );
                                        let ch = if (0x20..=0x7E).contains(&byte) {
                                            byte as char
                                        } else {
                                            '.'
                                        };
                                        ui.label(
                                            RichText::new(ch.to_string())
                                                .monospace()
                                                .color(color),
                                        );
                                    }
                                });

                                ui.end_row();
                            }
                        });
                });
        });
    }
}

impl<'a> Widget for TsPacketWidget<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            self.render_summary_bar(ui);
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            self.render_header_fields(ui);
            ui.add_space(4.0);
            self.render_adaptation_field(ui);
            ui.add_space(4.0);
            self.render_payload_section(ui);
            ui.add_space(4.0);
            self.render_hex_view(ui);
        })
        .response
    }
}

fn field_row(ui: &mut Ui, name: &str, value: impl Into<String>) {
    ui.label(RichText::new(name).monospace().strong());
    ui.label(RichText::new(value.into()).monospace());
    ui.end_row();
}

fn format_bool_flag(val: bool) -> String {
    if val {
        "1 (true)".to_owned()
    } else {
        "0 (false)".to_owned()
    }
}

fn result_bool_row(ui: &mut Ui, name: &str, res: Result<bool, TsError>) {
    ui.label(RichText::new(name).monospace().strong());
    match res {
        Ok(val) => {
            ui.label(RichText::new(format_bool_flag(val)).monospace());
        }
        Err(err) => {
            ui.label(RichText::new(err.to_string()).color(INVALID_COLOR));
        }
    }
    ui.end_row();
}

fn well_known_pid_name(pid: u16) -> Option<&'static str> {
    match pid {
        0x0000 => Some("Program Association Table (PAT)"),
        0x0001 => Some("Conditional Access Table (CAT)"),
        0x0002 => Some("Transport Stream Description Table (TSDT)"),
        0x0003 => Some("IPMP Control Information Table"),
        0x0010 => Some("Network Information Table (NIT)"),
        0x0011 => Some("Service Description Table (SDT) / BAT"),
        0x0012 => Some("Event Information Table (EIT)"),
        0x0013 => Some("Running Status Table (RST)"),
        0x0014 => Some("Time and Date Table (TDT) / TOT"),
        0x1FFF => Some("Null Packet"),
        _ => None,
    }
}

fn byte_region_style(
    byte_offset: usize,
    header_range: &std::ops::Range<usize>,
    af_range: Option<&std::ops::Range<usize>>,
    payload_range: Option<&std::ops::Range<usize>>,
) -> (Color32, &'static str) {
    if header_range.contains(&byte_offset) {
        (HEADER_COLOR, "TS Header")
    } else if af_range.is_some_and(|r| r.contains(&byte_offset)) {
        (ADAPTATION_COLOR, "Adaptation Field")
    } else if payload_range.is_some_and(|r| r.contains(&byte_offset)) {
        (PAYLOAD_COLOR, "Payload")
    } else {
        (Color32::GRAY, "Unassigned")
    }
}
