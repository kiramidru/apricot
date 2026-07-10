use crate::models::{Document, ReaderSettings};
use eframe::egui; // Import pure data types

use egui_phosphor_icons::{add_fonts, icons};
use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender, channel};

use crate::{models::Document, services::manga_parser::MangaReader, ui::components::CustomButton};

pub struct Apricot {
    sender: Sender<(usize, egui::ColorImage)>,
    receiver: Receiver<(usize, egui::ColorImage)>,

    active_document: Option<Document>,

    current_file_path: Option<String>,
    document_title: String,

    current_page: usize,
    total_pages: usize,
    texture_cache: Vec<Option<egui::TextureHandle>>,
    loading: HashSet<usize>,
}

impl Apricot {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        add_fonts(&mut fonts);
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        let reader = MangaReader::new("./test.cbz").expect("Failed to initialize manga reader");
        let cache_size = reader.file_names.len();
        let (sender, receiver) = channel();

        Self {
            document_title: "test.cbz".to_owned(),
            current_page: 1,
            total_pages: cache_size,
            texture_cache: vec![None; cache_size],
            sender,
            receiver,
            loading: HashSet::new(),
            active_document: Some(Document::Manga(reader)),
        }
    }

    fn handle_async_textures(&mut self, ctx: &egui::Context) {
        while let Ok((index, color_image)) = self.receiver.try_recv() {
            let handle =
                ctx.load_texture(format!("page_{}", index), color_image, Default::default());
            self.texture_cache[index] = Some(handle);
            self.loading.remove(&index);
        }
    }

    fn render_global_controls(&mut self, ctx: &egui::Context) {
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
    }

    fn render_local_controls(&mut self, ctx: &egui::Context) {
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
    }

    fn render_reader_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Some(document) = &self.active_document else {
                ui.centered_and_justified(|ui| {
                    ui.label("No document open.");
                });
                return;
            };

            match document {
                Document::Manga(reader) => self.render_manga_view(ui, ctx, reader),
                Document::Pdf { .. } => {
                    ui.label("PDF view not implemented yet");
                }
                Document::Epub { .. } => {
                    ui.label("EPUB view not implemented yet");
                }
            }
        });
    }

    fn render_manga_view(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, reader: &MangaReader) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for i in 0..reader.file_names.len() {
                if let Some(texture) = &self.texture_cache[i] {
                    let available_width = ui.available_width();
                    ui.add(
                        egui::Image::new(&*texture)
                            .fit_to_exact_size(egui::Vec2::new(available_width, f32::INFINITY))
                            .maintain_aspect_ratio(true),
                    );
                } else if !self.loading.contains(&i) {
                    self.loading.insert(i);
                    let sender = self.sender.clone();
                    let bytes = reader.get_page(i).expect("Failed to load page");

                    rayon::spawn(move || {
                        let img = image::load_from_memory(&bytes).expect("Decode fail");
                        let scaled = img.resize(800, 2000, image::imageops::FilterType::Triangle);
                        let size = [scaled.width() as usize, scaled.height() as usize];
                        let rgba = scaled.to_rgba8();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            rgba.as_flat_samples().as_slice(),
                        );
                        sender.send((i, color_image)).ok();
                    });
                }
            }
        });
    }
}

impl eframe::App for Apricot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_async_textures(ctx);
        self.render_global_controls(ctx);
        self.render_local_controls(ctx);
        self.render_reader_canvas(ctx);
    }
}
