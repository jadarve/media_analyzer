use std::rc::Rc;

use crate::loader::{FileLoader, TsContainer};
use crate::ui::{panels::AppPanels, theme, views::TsView, Panel, View};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MediaAnalyzerApp {
    #[serde(skip)]
    loader: FileLoader,

    #[serde(skip)]
    container: Option<Rc<TsContainer>>,

    #[serde(skip)]
    panels: AppPanels,

    #[serde(skip)]
    ts_view: TsView,
}

impl MediaAnalyzerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize the look and feel of egui:
        theme::apply(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for MediaAnalyzerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(result) = self.loader.poll() {
            match result {
                Ok(loaded_file) => {
                    let container = Rc::new(TsContainer::from_loaded_file(loaded_file));
                    self.panels.bottom_panel.set_status(format!(
                        "Loaded {} — {} bytes ({} packets)",
                        container.name(),
                        container.byte_len(),
                        container.len()
                    ));
                    self.panels.left_panel.set_container(Rc::clone(&container));
                    self.container = Some(container);
                }
                Err(err) => {
                    self.panels.bottom_panel.set_error(err);
                }
            }
        }

        self.panels.show(ui);

        if self.panels.left_toolbar.take_open_file_request() {
            self.loader.open_file_dialog(ui.ctx());
        }

        if self.panels.left_panel.take_selection_changed() {
            let selected_pkt = self
                .panels
                .left_panel
                .selected_packet()
                .and_then(|idx| self.container.as_ref()?.packet_at(idx));
            self.ts_view.set_packet(selected_pkt);
        }

        egui::CentralPanel::default().show(ui, |ui| {
            self.ts_view.ui(ui);
        });
    }
}
