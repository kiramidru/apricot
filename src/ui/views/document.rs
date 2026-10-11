use crate::ui::theme::{colors, typo};
use eframe::egui::{self, Stroke, TextureHandle, Ui, Vec2};

pub struct DocumentContent {
    emblem_texture: Option<TextureHandle>,
}

impl Default for DocumentContent {
    fn default() -> Self {
        Self {
            emblem_texture: None,
        }
    }
}

impl DocumentContent {
    fn ensure_texture(&mut self, ctx: &egui::Context) -> &TextureHandle {
        if self.emblem_texture.is_none() {
            let raw_rgba = include_bytes!("../../../assets/emblem.rgba");
            let color_image = egui::ColorImage::from_rgba_unmultiplied([83, 65], raw_rgba);
            let handle = ctx.load_texture("emblem", color_image, Default::default());
            self.emblem_texture = Some(handle);
        }
        self.emblem_texture.as_ref().unwrap()
    }

    pub fn render(&mut self, ui: &mut Ui, available_width: f32) {
        let texture = self.ensure_texture(ui.ctx()).clone();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Center the reading column with comfortable page margins
                    let content_w = (available_width.min(540.0) - 56.0).max(320.0);
                    let side_pad = ((available_width - content_w) * 0.5).max(28.0);

                    ui.add_space(side_pad);

                    ui.vertical(|ui| {
                        ui.set_max_width(content_w);
                        ui.add_space(24.0);

                        // 1. Australian Coat of Arms Emblem (left-aligned with text)
                        ui.add(
                            egui::Image::new(&texture)
                                .fit_to_exact_size(Vec2::new(92.0, 72.0))
                                .maintain_aspect_ratio(true),
                        );
                        ui.add_space(16.0);

                        // 2. Title
                        ui.label(
                            egui::RichText::new("Banking, Insurance, Life Insurance, Health\nInsurance and Superannuation (prudential\nstandard) determination No. 2 of 2023")
                                .font(typo::doc_title())
                                .color(colors::DOC_TITLE),
                        );
                        ui.add_space(12.0);

                        // 3. Subtitle
                        ui.label(
                            egui::RichText::new("Prudential Standard CPS 230 Operational Risk Management")
                                .font(typo::doc_subtitle())
                                .color(colors::DOC_SUBTITLE),
                        );
                        ui.add_space(12.0);

                        // 4. Legislation list (in italics)
                        let acts = [
                            "Banking Act 1959",
                            "Insurance Act 1973",
                            "Life Insurance Act 1995",
                            "Private Health Insurance (Prudential Supervision) Act 2015",
                            "Superannuation Industry (Supervision) Act 1993",
                        ];
                        for act in acts {
                            ui.label(
                                egui::RichText::new(act)
                                    .font(typo::doc_legislation())
                                    .color(colors::DOC_LEGISLATION),
                            );
                        }
                        ui.add_space(12.0);

                        // 5. Thin horizontal rule
                        let (rule_rect, _) = ui.allocate_exact_size(Vec2::new(content_w, 1.0), egui::Sense::hover());
                        ui.painter().line_segment(
                            [rule_rect.left_top(), rule_rect.right_top()],
                            Stroke::new(1.0_f32, colors::DOC_RULE),
                        );
                        ui.add_space(14.0);

                        // 6. Body Preamble
                        ui.label(
                            egui::RichText::new("I, John Lonsdale, a delegate of APRA:")
                                .font(typo::doc_body())
                                .color(colors::DOC_BODY),
                        );
                        ui.add_space(10.0);

                        // 7. Clauses (a) through (e) with legal hanging indent
                        let clauses = [
                            ("(a)", "under subsections 11AF(3) of the Banking Act, 32(4) of the Insurance Act, and 230A(5) of the Life Insurance Act, REVOKE Banking, Insurance and Life Insurance (prudential standard) determination No. 6 of 2016 including Prudential Standard CPS 231 Outsourcing made under that determination;"),
                            ("(b)", "under subsections 11AF(3) of the Banking Act, 32(4) of the Insurance Act, and 230A(5) of the Life Insurance Act, REVOKE Banking, Insurance and Life Insurance (prudential standard) determination No. 7 of 2016 including Prudential Standard CPS 232 Business Continuity Management made under that determination;"),
                            ("(c)", "under subsection 92(5) of the PHIPS Act, REVOKE Health Insurance (prudential standard) determination No. 4 of 2015, including Prudential Standard HPS 231 Outsourcing made under that determination;"),
                            ("(d)", "under subsection 34C(6) of the SIS Act, REVOKE Superannuation (prudential standard) determination No. 3 of 2012 including, Prudential Standard SPS 231 Outsourcing made under that determination, and Superannuation (prudential standard) determination No. 4 of 2012 including Prudential Standard SPS 232 Business Continuity Management made under that determination;"),
                            ("(e)", "under subsection 11AF(1) of the Banking Act, DETERMINE the prudential"),
                        ];

                        let marker_w = 24.0_f32;
                        let text_w = content_w - marker_w - 6.0;

                        for (marker, text) in clauses {
                            ui.horizontal_top(|ui| {
                                ui.allocate_ui(Vec2::new(marker_w, 16.0), |ui| {
                                    ui.label(
                                        egui::RichText::new(marker)
                                            .font(typo::doc_body())
                                            .color(colors::DOC_BODY),
                                    );
                                });
                                ui.add_space(4.0);
                                ui.allocate_ui(Vec2::new(text_w, 16.0), |ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(text)
                                                .font(typo::doc_body())
                                                .color(colors::DOC_BODY),
                                        )
                                        .wrap(),
                                    );
                                });
                            });
                            ui.add_space(12.0);
                        }

                        ui.add_space(48.0);
                    });

                    ui.add_space(side_pad);
                });
            });
    }
}
