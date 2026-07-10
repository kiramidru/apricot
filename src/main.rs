mod services;
mod ui;

use eframe::egui;
<<<<<<< HEAD
use egui_phosphor_icons::{add_fonts, icons};

use crate::ui::CustomButton;
=======
use ui::Apricot;
>>>>>>> 40d0dee (init commit)

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Apricot",
        native_options,
        Box::new(|cc| Ok(Box::new(Apricot::new(cc)))),
    )
}
<<<<<<< HEAD

struct Apricot {
    document_title: String,
    current_page: usize,
    total_pages: usize,
    pdf_content: Option<String>,
    fonts_loaded: bool,
}

impl Apricot {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        add_fonts(&mut fonts);
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        Self {
            document_title: "No document loaded".to_owned(),
            current_page: 1,
            total_pages: 100,
            pdf_content: None,
            fonts_loaded: true,
        }
    }
}

impl eframe::App for Apricot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("global_controls")
            .frame(egui::Frame {
                fill: ctx.style().visuals.panel_fill,
                inner_margin: egui::Margin::symmetric(10, 10),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(CustomButton::new(icons::ARROW_LEFT).border(false));
                        ui.label(icons::LINE_VERTICAL);
                        ui.label(&self.document_title);
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(CustomButton::new(icons::INFO));
                        ui.add(CustomButton::new(icons::BOOKMARK_SIMPLE));
                    });
                });
            });

        egui::TopBottomPanel::top("local_controls")
            .frame(egui::Frame {
                fill: ctx.style().visuals.panel_fill,
                inner_margin: egui::Margin::symmetric(10, 10),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.columns(3, |columns| {
                    columns[0].horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add(CustomButton::new(icons::MAGNIFYING_GLASS).border(false));
                            ui.add(CustomButton::new(icons::CARET_UP).border(false));
                            ui.label(format!("{} / {}", self.current_page, self.total_pages));
                            ui.add(CustomButton::new(icons::CARET_DOWN).border(false));
                        });
                    });

                    columns[1].horizontal(|ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label("Apricot Reader");
                        });
                    });

                    columns[2].horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(CustomButton::new(icons::DOTS_THREE).border(false));
                            ui.add(CustomButton::new(icons::PRINTER).border(false));
                            ui.add(CustomButton::new(icons::ARROWS_OUT_SIMPLE).border(false));
                        });
                    });
                });
            });

        // Central Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref text) = self.pdf_content {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::Label::new(text).selectable(true));
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Please open a PDF file to begin.");
                });
            }
        });
    }
}
=======
>>>>>>> 40d0dee (init commit)
