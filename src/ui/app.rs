use crate::ui::components::{
    render_toolbar, render_topbar, ToolbarAction, TopBarAction, TopBarMode,
};
use crate::ui::theme::{colors, configure_visuals, setup_fonts};
use crate::ui::views::{
    render_settings, AppView, DocumentContent, HomeAction, HomeView, SettingsAction, SettingsState,
};
use eframe::egui::{self, Pos2, Rect, Stroke, Vec2};

pub struct Apricot {
    view: AppView,
    previous_view: Option<AppView>,
    home: HomeView,
    document: DocumentContent,
    settings: SettingsState,
    current_page: usize,
    total_pages: usize,
    zoom_pct: usize,
}

impl Apricot {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        configure_visuals(&cc.egui_ctx);

        Self {
            view: AppView::Home,
            previous_view: None,
            home: HomeView::default(),
            document: DocumentContent::default(),
            settings: SettingsState::default(),
            current_page: 1,
            total_pages: 13,
            zoom_pct: 100,
        }
    }

    fn navigate_to_settings(&mut self) {
        if !matches!(self.view, AppView::Settings) {
            self.previous_view = Some(self.view.clone());
            self.view = AppView::Settings;
        }
    }

    fn navigate_back(&mut self) {
        if let Some(prev) = self.previous_view.take() {
            self.view = prev;
        } else {
            self.view = AppView::Home;
        }
    }
}

impl eframe::App for Apricot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(colors::CARD_BG))
            .show(ctx, |ui| {
                let avail = ui.available_size();
                let origin = ui.min_rect().min;

                // --- 1. Top Bar Component ---
                let topbar_h = 48.0_f32;
                let topbar_rect = Rect::from_min_size(origin, Vec2::new(avail.x, topbar_h));
                let mut topbar_ui = ui.new_child(egui::UiBuilder::new().max_rect(topbar_rect));

                let mode = match &self.view {
                    AppView::Home => TopBarMode::Home,
                    AppView::Document { title, .. } => TopBarMode::Document { title },
                    AppView::Settings => TopBarMode::Settings,
                };

                match render_topbar(&mut topbar_ui, mode, topbar_rect) {
                    TopBarAction::None => {}
                    TopBarAction::Back => self.navigate_back(),
                    TopBarAction::NavigateHome => self.view = AppView::Home,
                    TopBarAction::NavigateSettings => self.navigate_to_settings(),
                    TopBarAction::OpenFile => self.home.open_file_dialog(ui.ctx().clone()),
                }

                // Divider 1
                let div1_y = origin.y + topbar_h;
                ui.painter().line_segment(
                    [Pos2::new(origin.x, div1_y), Pos2::new(origin.x + avail.x, div1_y)],
                    Stroke::new(1.0_f32, colors::DIVIDER),
                );

                // --- 2. Views Router ---
                match &self.view {
                    AppView::Home => {
                        let home_h = (avail.y - topbar_h).max(100.0);
                        let home_rect = Rect::from_min_size(Pos2::new(origin.x, div1_y), Vec2::new(avail.x, home_h));
                        let mut home_ui = ui.new_child(egui::UiBuilder::new().max_rect(home_rect));

                        if let HomeAction::OpenDocument { title, path } = self.home.render(&mut home_ui, avail.x) {
                            self.view = AppView::Document { title, path };
                        }
                    }
                    AppView::Document { .. } => {
                        // Toolbar Component
                        let toolbar_h = 38.0_f32;
                        let toolbar_rect = Rect::from_min_size(Pos2::new(origin.x, div1_y), Vec2::new(avail.x, toolbar_h));
                        let mut toolbar_ui = ui.new_child(egui::UiBuilder::new().max_rect(toolbar_rect));

                        match render_toolbar(
                            &mut toolbar_ui,
                            toolbar_rect,
                            self.current_page,
                            self.total_pages,
                            self.zoom_pct,
                        ) {
                            ToolbarAction::None => {}
                            ToolbarAction::PrevPage => {
                                self.current_page = (self.current_page.saturating_sub(1)).max(1);
                            }
                            ToolbarAction::NextPage => {
                                self.current_page = (self.current_page + 1).min(self.total_pages);
                            }
                            ToolbarAction::ZoomIn => {
                                self.zoom_pct = (self.zoom_pct + 10).min(300);
                            }
                            ToolbarAction::ZoomOut => {
                                self.zoom_pct = self.zoom_pct.saturating_sub(10).max(25);
                            }
                            ToolbarAction::SetZoom(z) => {
                                self.zoom_pct = z;
                            }
                        }

                        // Divider 2
                        let div2_y = div1_y + toolbar_h;
                        ui.painter().line_segment(
                            [Pos2::new(origin.x, div2_y), Pos2::new(origin.x + avail.x, div2_y)],
                            Stroke::new(1.0_f32, colors::DIVIDER),
                        );

                        // Document Canvas View
                        let doc_h = (avail.y - (topbar_h + toolbar_h)).max(100.0);
                        let doc_rect = Rect::from_min_size(Pos2::new(origin.x, div2_y), Vec2::new(avail.x, doc_h));
                        let mut doc_ui = ui.new_child(egui::UiBuilder::new().max_rect(doc_rect));
                        self.document.render(&mut doc_ui, avail.x);
                    }
                    AppView::Settings => {
                        let settings_h = (avail.y - topbar_h).max(100.0);
                        let settings_rect = Rect::from_min_size(Pos2::new(origin.x, div1_y), Vec2::new(avail.x, settings_h));
                        let mut settings_ui = ui.new_child(egui::UiBuilder::new().max_rect(settings_rect));

                        match render_settings(&mut settings_ui, &mut self.settings, avail.x) {
                            SettingsAction::None => {}
                            SettingsAction::Back => self.navigate_back(),
                            SettingsAction::ClearRecentHistory => self.home.recent_docs.clear(),
                        }
                    }
                }
            });
    }
}
