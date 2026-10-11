use crate::ui::components::widgets::badge_icon;
use crate::ui::theme::{colors, typo};
use eframe::egui::{self, CornerRadius, FontId, Sense, Stroke, Ui};
use egui_phosphor_icons::icons;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct RecentDocItem {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub pages_str: &'static str,
    pub path: Option<&'static str>,
}

pub struct HomeView {
    pub recent_docs: Vec<RecentDocItem>,
    sender: Sender<PathBuf>,
    receiver: Receiver<PathBuf>,
}

impl Default for HomeView {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            recent_docs: vec![
                RecentDocItem {
                    title: "CPS 234 Information Security",
                    subtitle: "APRA Standards • Prudential Standard CPS 230",
                    pages_str: "13 pages",
                    path: Some("CPS 234 Information Security.pdf"),
                },
                RecentDocItem {
                    title: "Banking Act Determination No. 2 of 2023",
                    subtitle: "Australian Prudential Regulation Authority",
                    pages_str: "6 pages",
                    path: Some("Determination No. 2 of 2023.pdf"),
                },
                RecentDocItem {
                    title: "Operational Risk Management PPG 230",
                    subtitle: "APRA Prudential Practice Guide",
                    pages_str: "24 pages",
                    path: Some("PPG 230 Operational Risk.pdf"),
                },
            ],
            sender,
            receiver,
        }
    }
}

pub enum HomeAction {
    None,
    OpenDocument { title: String, path: Option<String> },
}

impl HomeView {
    pub fn open_file_dialog(&self, ctx: egui::Context) {
        let tx = self.sender.clone();
        std::thread::spawn(move || {
            if let Some(file) = rfd::FileDialog::new()
                .add_filter("Documents", &["pdf", "epub", "cbz", "txt", "md"])
                .pick_file()
            {
                tx.send(file).ok();
                ctx.request_repaint();
            }
        });
    }

    pub fn render(&mut self, ui: &mut Ui, available_width: f32) -> HomeAction {
        let mut action = HomeAction::None;

        // Check for drag & drop file events
        let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        if let Some(file) = dropped_files.first() {
            if let Some(path) = &file.path {
                let title = path.file_name().and_then(|n| n.to_str()).unwrap_or("Document").to_string();
                return HomeAction::OpenDocument {
                    title,
                    path: Some(path.to_string_lossy().to_string()),
                };
            }
        }

        // Check if a file was picked via dialog
        while let Ok(path) = self.receiver.try_recv() {
            let title = path.file_name().and_then(|n| n.to_str()).unwrap_or("Document").to_string();
            return HomeAction::OpenDocument {
                title,
                path: Some(path.to_string_lossy().to_string()),
            };
        }

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

                        // --- 1. Recent Documents Header ---
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Recent Documents")
                                    .font(typo::doc_title())
                                    .color(colors::TEXT_PRIMARY),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let count_str = format!("{} items", self.recent_docs.len());
                                ui.label(
                                    egui::RichText::new(count_str)
                                        .font(typo::breadcrumb())
                                        .color(colors::TEXT_MUTED),
                                );
                            });
                        });
                        ui.add_space(14.0);

                        // --- 2. Recent Document Items with Apricot styling ---
                        for doc in &self.recent_docs {
                            let frame = egui::Frame::NONE
                                .fill(colors::CARD_BG)
                                .stroke(Stroke::new(1.0_f32, colors::BTN_BORDER))
                                .corner_radius(CornerRadius::same(8))
                                .inner_margin(egui::Margin::symmetric(16, 14));

                            let item_resp = frame
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Badge icon box with apricot tint
                                        badge_icon(
                                            ui,
                                            icons::FILE_TEXT.0,
                                            16.0,
                                            32.0,
                                            colors::APRICOT_SOFT,
                                            colors::APRICOT_BORDER,
                                            colors::APRICOT_DARK,
                                        );

                                        ui.add_space(10.0);

                                        // Text column
                                        ui.vertical(|ui| {
                                            ui.label(
                                                egui::RichText::new(doc.title)
                                                    .font(typo::toolbar_text())
                                                    .color(colors::TEXT_PRIMARY),
                                            );
                                            ui.add_space(2.0);
                                            ui.label(
                                                egui::RichText::new(doc.subtitle)
                                                    .font(typo::breadcrumb())
                                                    .color(colors::TEXT_MUTED),
                                            );
                                        });

                                        // Right metadata + chevron
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let chev_font = FontId::new(11.0_f32, egui::FontFamily::Name("phosphor-regular".into()));
                                            ui.label(
                                                egui::RichText::new(icons::CARET_RIGHT.0)
                                                    .font(chev_font)
                                                    .color(colors::BREADCRUMB_CHEVRON),
                                            );
                                            ui.add_space(6.0);
                                            ui.label(
                                                egui::RichText::new(doc.pages_str)
                                                    .font(typo::breadcrumb())
                                                    .color(colors::TEXT_SUBTLE),
                                            );
                                        });
                                    });
                                })
                                .response;

                            // Click & hover interaction
                            let interact_rect = item_resp.rect;
                            let click_resp = ui.interact(interact_rect, ui.id().with(doc.title), Sense::click());
                            if click_resp.hovered() {
                                ui.painter().rect_stroke(
                                    interact_rect,
                                    CornerRadius::same(8),
                                    Stroke::new(1.2_f32, colors::APRICOT_BORDER),
                                    egui::StrokeKind::Inside,
                                );
                            }

                            if click_resp.clicked() {
                                action = HomeAction::OpenDocument {
                                    title: doc.title.to_string(),
                                    path: doc.path.map(|s| s.to_string()),
                                };
                            }

                            ui.add_space(10.0);
                        }

                        ui.add_space(40.0);
                    });

                    ui.add_space(side_pad);
                });
            });

        action
    }
}
