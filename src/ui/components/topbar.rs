use super::widgets::{bordered_icon_btn, icon_btn};
use crate::ui::theme::{colors, typo};
use eframe::egui::{self, CornerRadius, FontId, Rect, Sense, Stroke, Ui, Vec2};
use egui_phosphor_icons::icons;

#[derive(Debug, Clone)]
pub enum TopBarMode<'a> {
    Home,
    Document { title: &'a str },
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBarAction {
    None,
    Back,
    NavigateHome,
    NavigateSettings,
    OpenFile,
}

pub fn render_topbar(ui: &mut Ui, mode: TopBarMode<'_>, rect: Rect) -> TopBarAction {
    let mut action = TopBarAction::None;

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.horizontal_centered(|ui| {
            ui.add_space(14.0);

            match mode {
                TopBarMode::Home => {
                    // Brand emblem: warm apricot icon
                    ui.label(
                        egui::RichText::new(icons::FOLDER_SIMPLE.0)
                            .font(FontId::new(17.0_f32, egui::FontFamily::Name("phosphor-regular".into())))
                            .color(colors::APRICOT_DARK),
                    );
                    ui.add_space(4.0);

                    ui.label(
                        egui::RichText::new("Documents")
                            .font(typo::doc_subtitle())
                            .color(colors::TEXT_PRIMARY),
                    );

                    // Right: Settings gear & Open File button
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(14.0);

                        // Settings gear button
                        if bordered_icon_btn(ui, icons::GEAR_SIX, 14.0, 24.0).clicked() {
                            action = TopBarAction::NavigateSettings;
                        }
                        ui.add_space(6.0);

                        // "Open File" button
                        let btn = egui::Button::new(
                            egui::RichText::new(format!("{}  Open File", icons::FOLDER_OPEN.0))
                                .font(typo::toolbar_text())
                                .color(colors::TEXT_PRIMARY),
                        )
                        .fill(colors::CARD_BG)
                        .stroke(Stroke::new(1.0_f32, colors::BTN_BORDER))
                        .corner_radius(CornerRadius::same(5))
                        .min_size(Vec2::new(96.0, 26.0));

                        if ui.add(btn).clicked() {
                            action = TopBarAction::OpenFile;
                        }
                    });
                }
                TopBarMode::Document { title } => {
                    // Back arrow button
                    if icon_btn(ui, icons::ARROW_LEFT, 15.0, 24.0).clicked() {
                        action = TopBarAction::Back;
                        return;
                    }
                    ui.add_space(6.0);

                    // Breadcrumb segment 1: Documents (clickable)
                    let doc_btn = ui.add(
                        egui::Label::new(
                            egui::RichText::new("Documents")
                                .font(typo::breadcrumb())
                                .color(colors::TEXT_MUTED),
                        )
                        .sense(Sense::click()),
                    );
                    if doc_btn.clicked() {
                        action = TopBarAction::NavigateHome;
                        return;
                    }
                    if doc_btn.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    // Chevron separator
                    render_breadcrumb_separator(ui);

                    // Breadcrumb segment 2: Team
                    ui.label(
                        egui::RichText::new("Team")
                            .font(typo::breadcrumb())
                            .color(colors::TEXT_MUTED),
                    );

                    render_breadcrumb_separator(ui);

                    // Breadcrumb segment 3: APRA Standards
                    ui.label(
                        egui::RichText::new("APRA Standards")
                            .font(typo::breadcrumb())
                            .color(colors::TEXT_MUTED),
                    );

                    render_breadcrumb_separator(ui);

                    // Active document title
                    let display_title = if title.len() > 30 { format!("{}...", &title[..28]) } else { title.to_string() };
                    ui.label(
                        egui::RichText::new(display_title)
                            .font(typo::breadcrumb_active())
                            .color(colors::TEXT_PRIMARY),
                    );

                    // Right buttons: Bookmark & Settings
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(14.0);

                        // Bookmark button
                        bordered_icon_btn(ui, icons::BOOKMARK_SIMPLE, 14.0, 24.0);
                        ui.add_space(6.0);

                        // Settings gear button
                        if bordered_icon_btn(ui, icons::GEAR_SIX, 14.0, 24.0).clicked() {
                            action = TopBarAction::NavigateSettings;
                        }
                    });
                }
                TopBarMode::Settings => {
                    // Back arrow button
                    if icon_btn(ui, icons::ARROW_LEFT, 15.0, 24.0).clicked() {
                        action = TopBarAction::Back;
                        return;
                    }
                    ui.add_space(6.0);

                    // Breadcrumb: Documents (clickable) › Settings
                    let doc_btn = ui.add(
                        egui::Label::new(
                            egui::RichText::new("Documents")
                                .font(typo::breadcrumb())
                                .color(colors::TEXT_MUTED),
                        )
                        .sense(Sense::click()),
                    );
                    if doc_btn.clicked() {
                        action = TopBarAction::NavigateHome;
                        return;
                    }
                    if doc_btn.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    render_breadcrumb_separator(ui);

                    ui.label(
                        egui::RichText::new("Settings")
                            .font(typo::breadcrumb_active())
                            .color(colors::TEXT_PRIMARY),
                    );
                }
            }
        });
    });

    action
}

fn render_breadcrumb_separator(ui: &mut Ui) {
    ui.add_space(2.0);
    ui.label(
        egui::RichText::new("›")
            .font(FontId::new(13.0_f32, egui::FontFamily::Proportional))
            .color(colors::BREADCRUMB_CHEVRON),
    );
    ui.add_space(2.0);
}
