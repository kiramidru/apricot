use crate::ui::components::widgets::{choice_pill, section_header, subtle_divider};
use crate::ui::theme::{colors, typo};
use eframe::egui::{self, Color32, CornerRadius, Stroke, Ui, Vec2};
use egui_phosphor_icons::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageLayoutMode {
    Continuous,
    SinglePage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingDirection {
    LeftToRight,
    RightToLeft,
}

pub struct SettingsState {
    pub theme: ThemeMode,
    pub default_zoom: usize,
    pub font_family: usize, // 0 = Sans-serif, 1 = Serif
    pub layout_mode: PageLayoutMode,
    pub reading_direction: ReadingDirection,
    pub remember_position: bool,
    pub library_path: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Light,
            default_zoom: 100,
            font_family: 0,
            layout_mode: PageLayoutMode::Continuous,
            reading_direction: ReadingDirection::LeftToRight,
            remember_position: true,
            library_path: "~/Documents".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SettingsAction {
    None,
    Back,
    ClearRecentHistory,
}

pub fn render_settings(ui: &mut Ui, state: &mut SettingsState, available_width: f32) -> SettingsAction {
    let mut action = SettingsAction::None;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let content_w = (available_width.min(540.0) - 56.0).max(320.0);
                let side_pad = ((available_width - content_w) * 0.5).max(28.0);

                ui.add_space(side_pad);

                ui.vertical(|ui| {
                    ui.set_max_width(content_w);
                    ui.add_space(20.0);

                    // --- Header: Settings title ---
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Settings")
                                .font(typo::doc_title())
                                .color(colors::TEXT_PRIMARY),
                        );
                    });
                    ui.add_space(14.0);

                    // ==========================================
                    // Section 1: Appearance
                    // ==========================================
                    section_header(ui, icons::PALETTE.0, "Appearance");
                    ui.add_space(8.0);

                    egui::Frame::NONE
                        .fill(colors::CARD_BG)
                        .stroke(Stroke::new(1.0_f32, colors::BTN_BORDER))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(16, 14))
                        .show(ui, |ui| {
                            // Row 1: Theme
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Theme")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let options = [
                                        (ThemeMode::System, "System"),
                                        (ThemeMode::Dark, "Dark"),
                                        (ThemeMode::Light, "Light"),
                                    ];
                                    for (mode, label) in options {
                                        let is_sel = state.theme == mode;
                                        if choice_pill(ui, label, is_sel) {
                                            state.theme = mode;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            });

                            ui.add_space(10.0);
                            subtle_divider(ui);
                            ui.add_space(10.0);

                            // Row 2: Default Zoom
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Default Zoom")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let zooms = [150, 125, 100];
                                    for z in zooms {
                                        let is_sel = state.default_zoom == z;
                                        let label = format!("{}%", z);
                                        if choice_pill(ui, &label, is_sel) {
                                            state.default_zoom = z;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            });

                            ui.add_space(10.0);
                            subtle_divider(ui);
                            ui.add_space(10.0);

                            // Row 3: Reader Typography
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Typography")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let fonts = [(1, "Serif"), (0, "Sans-serif")];
                                    for (idx, label) in fonts {
                                        let is_sel = state.font_family == idx;
                                        if choice_pill(ui, label, is_sel) {
                                            state.font_family = idx;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            });
                        });

                    ui.add_space(20.0);

                    // ==========================================
                    // Section 2: Reading Preferences
                    // ==========================================
                    section_header(ui, icons::BOOK_OPEN.0, "Reading Preferences");
                    ui.add_space(8.0);

                    egui::Frame::NONE
                        .fill(colors::CARD_BG)
                        .stroke(Stroke::new(1.0_f32, colors::BTN_BORDER))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(16, 14))
                        .show(ui, |ui| {
                            // Row 1: Layout Mode
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Page Layout")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let layouts = [
                                        (PageLayoutMode::SinglePage, "Single Page"),
                                        (PageLayoutMode::Continuous, "Continuous Scroll"),
                                    ];
                                    for (mode, label) in layouts {
                                        let is_sel = state.layout_mode == mode;
                                        if choice_pill(ui, label, is_sel) {
                                            state.layout_mode = mode;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            });

                            ui.add_space(10.0);
                            subtle_divider(ui);
                            ui.add_space(10.0);

                            // Row 2: Reading Direction
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Direction")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let dirs = [
                                        (ReadingDirection::RightToLeft, "Manga (RTL)"),
                                        (ReadingDirection::LeftToRight, "Standard (LTR)"),
                                    ];
                                    for (mode, label) in dirs {
                                        let is_sel = state.reading_direction == mode;
                                        if choice_pill(ui, label, is_sel) {
                                            state.reading_direction = mode;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            });

                            ui.add_space(10.0);
                            subtle_divider(ui);
                            ui.add_space(10.0);

                            // Row 3: Remember Position
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Remember Reading Progress")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let toggle_label = if state.remember_position { "Enabled" } else { "Disabled" };
                                    if choice_pill(ui, toggle_label, state.remember_position) {
                                        state.remember_position = !state.remember_position;
                                    }
                                });
                            });
                        });

                    ui.add_space(20.0);

                    // ==========================================
                    // Section 3: Storage & Library
                    // ==========================================
                    section_header(ui, icons::FOLDER.0, "Storage & Library");
                    ui.add_space(8.0);

                    egui::Frame::NONE
                        .fill(colors::CARD_BG)
                        .stroke(Stroke::new(1.0_f32, colors::BTN_BORDER))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(16, 14))
                        .show(ui, |ui| {
                            // Row 1: Default Library Directory
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new("Default Documents Directory")
                                            .font(typo::toolbar_text())
                                            .color(colors::TEXT_PRIMARY),
                                    );
                                    ui.label(
                                        egui::RichText::new(&state.library_path)
                                            .font(typo::breadcrumb())
                                            .color(colors::TEXT_MUTED),
                                    );
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let btn = egui::Button::new(
                                        egui::RichText::new("Browse")
                                            .font(typo::toolbar_text())
                                            .color(colors::TEXT_PRIMARY),
                                    )
                                    .fill(colors::BTN_HOVER_BG)
                                    .stroke(Stroke::new(1.0_f32, colors::BTN_BORDER))
                                    .corner_radius(CornerRadius::same(5))
                                    .min_size(Vec2::new(64.0, 24.0));

                                    if ui.add(btn).clicked() {
                                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                            state.library_path = folder.to_string_lossy().to_string();
                                        }
                                    }
                                });
                            });

                            ui.add_space(10.0);
                            subtle_divider(ui);
                            ui.add_space(10.0);

                            // Row 2: Clear History
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("Clear Recent History")
                                        .font(typo::toolbar_text())
                                        .color(colors::TEXT_PRIMARY),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let btn = egui::Button::new(
                                        egui::RichText::new("Clear")
                                            .font(typo::toolbar_text())
                                            .color(Color32::from_rgb(0xC8, 0x30, 0x30)),
                                    )
                                    .fill(Color32::from_rgb(0xFF, 0xF5, 0xF5))
                                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(0xF0, 0xD0, 0xD0)))
                                    .corner_radius(CornerRadius::same(5))
                                    .min_size(Vec2::new(56.0, 24.0));

                                    if ui.add(btn).clicked() {
                                        action = SettingsAction::ClearRecentHistory;
                                    }
                                });
                            });
                        });

                    ui.add_space(40.0);
                });

                ui.add_space(side_pad);
            });
        });

    action
}
