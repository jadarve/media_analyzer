mod bottom_panel;
mod left_panel;
mod left_toolbar;
mod right_panel;
mod top_bar;

pub(crate) use bottom_panel::BottomPanel;
pub(crate) use left_panel::LeftSidePanel;
pub(crate) use left_toolbar::LeftToolbarPanel;
pub(crate) use right_panel::RightSidePanel;
pub(crate) use top_bar::TopBarPanel;

use crate::ui::Panel;

/// Coordinates all peripheral application shell panels around the central workspace.
#[derive(Default)]
pub(crate) struct AppPanels {
    pub(crate) top_bar: TopBarPanel,
    pub(crate) bottom_panel: BottomPanel,
    pub(crate) left_toolbar: LeftToolbarPanel,
    pub(crate) left_panel: LeftSidePanel,
    pub(crate) right_panel: RightSidePanel,
}

impl Panel for AppPanels {
    fn show(&mut self, ui: &mut egui::Ui) {
        self.top_bar.show(ui);
        self.bottom_panel.show(ui);
        self.left_toolbar.show(ui);
        self.left_panel.show(ui);
        self.right_panel.show(ui);
    }
}
