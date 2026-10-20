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
    fn show(&mut self, ctx: &egui::Context) {
        self.top_bar.show(ctx);
        self.bottom_panel.show(ctx);
        self.left_toolbar.show(ctx);
        self.left_panel.show(ctx);
        self.right_panel.show(ctx);
    }
}

