#![allow(dead_code)]
use crate::ui::theme::{colors, typo};
use eframe::egui::{self, Color32, CornerRadius, FontId, Pos2, Response, Sense, Stroke, Ui, Vec2};
use egui_phosphor_icons::Icon;

/// A borderless icon button with hover feedback
pub fn icon_btn(ui: &mut Ui, icon: Icon, icon_size: f32, box_size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(box_size), Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let hovered = response.hovered();

        if hovered {
            painter.rect_filled(rect, CornerRadius::same(5), colors::BTN_HOVER_BG);
        }

        let col = if hovered {
            colors::ICON_BTN_HOVER
        } else {
            colors::ICON_BTN
        };

        let font_id = FontId::new(icon_size, egui::FontFamily::Name("phosphor-regular".into()));
        painter.text(rect.center(), egui::Align2::CENTER_CENTER, icon.0, font_id, col);
    }
    response
}

/// A rounded button with a subtle border (used for Bookmark, Settings, etc.)
pub fn bordered_icon_btn(ui: &mut Ui, icon: Icon, icon_size: f32, box_size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(box_size), Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let hovered = response.hovered();

        let bg = if hovered { colors::BTN_HOVER_BG } else { colors::CARD_BG };
        painter.rect_filled(rect, CornerRadius::same(5), bg);
        painter.rect_stroke(
            rect,
            CornerRadius::same(5),
            Stroke::new(1.0_f32, colors::BTN_BORDER),
            egui::StrokeKind::Inside,
        );

        let col = if hovered {
            colors::ICON_BTN_HOVER
        } else {
            colors::ICON_BTN
        };

        let font_id = FontId::new(icon_size, egui::FontFamily::Name("phosphor-regular".into()));
        painter.text(rect.center(), egui::Align2::CENTER_CENTER, icon.0, font_id, col);
    }
    response
}

/// Page number box: rounded rectangle containing the page number
pub fn page_box(ui: &mut Ui, page: usize) -> Response {
    let size = Vec2::new(22.0, 22.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same(4), colors::PAGE_BOX_BG);
        painter.rect_stroke(
            rect,
            CornerRadius::same(4),
            Stroke::new(1.0_f32, colors::PAGE_BOX_BORDER),
            egui::StrokeKind::Inside,
        );

        let text = page.to_string();
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            typo::toolbar_text(),
            colors::TEXT_PRIMARY,
        );
    }
    response
}

/// Zoom dropdown pill showing "100% ▾", highlighted when open or hovered
pub fn zoom_dropdown(ui: &mut Ui, zoom_pct: usize, is_open: bool) -> Response {
    let size = Vec2::new(56.0, 22.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let hovered = response.hovered();

        let bg = if is_open {
            colors::APRICOT_SOFT
        } else if hovered {
            colors::BTN_HOVER_BG
        } else {
            colors::CARD_BG
        };

        painter.rect_filled(rect, CornerRadius::same(4), bg);

        let border_stroke = if is_open {
            Stroke::new(1.0_f32, colors::APRICOT_BORDER)
        } else {
            Stroke::new(1.0_f32, colors::BTN_BORDER)
        };
        painter.rect_stroke(rect, CornerRadius::same(4), border_stroke, egui::StrokeKind::Inside);

        let text = format!("{}%", zoom_pct);
        let center_y = rect.center().y;

        let text_col = if is_open { colors::APRICOT_DARK } else { colors::TEXT_PRIMARY };
        painter.text(
            Pos2::new(rect.center().x - 5.0, center_y),
            egui::Align2::CENTER_CENTER,
            text,
            typo::toolbar_text(),
            text_col,
        );

        let caret_icon = if is_open {
            egui_phosphor_icons::icons::CARET_UP.0
        } else {
            egui_phosphor_icons::icons::CARET_DOWN.0
        };
        let chev_font = FontId::new(8.5_f32, egui::FontFamily::Name("phosphor-regular".into()));
        painter.text(
            Pos2::new(rect.max.x - 8.0, center_y),
            egui::Align2::CENTER_CENTER,
            caret_icon,
            chev_font,
            if is_open { colors::APRICOT_DARK } else { colors::ICON_BTN },
        );
    }
    response
}

/// Selection / toggle pill with active Apricot accent styling
pub fn choice_pill(ui: &mut Ui, label: &str, is_selected: bool) -> bool {
    let (bg, stroke, text_col) = if is_selected {
        (
            colors::APRICOT_SOFT,
            Stroke::new(1.0_f32, colors::APRICOT_BORDER),
            colors::APRICOT_DARK,
        )
    } else {
        (
            colors::CARD_BG,
            Stroke::new(1.0_f32, colors::BTN_BORDER),
            colors::TEXT_MUTED,
        )
    };

    let btn = egui::Button::new(
        egui::RichText::new(label)
            .font(typo::breadcrumb())
            .color(text_col),
    )
    .fill(bg)
    .stroke(stroke)
    .corner_radius(CornerRadius::same(5))
    .min_size(Vec2::new(24.0, 22.0));

    ui.add(btn).clicked()
}

/// A thin horizontal divider line matching design tokens
pub fn subtle_divider(ui: &mut Ui) {
    let avail_w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, 1.0), Sense::hover());
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0_f32, colors::DIVIDER),
    );
}

/// Standardized section header with icon and title
pub fn section_header(ui: &mut Ui, icon_str: &str, title: &str) {
    ui.horizontal(|ui| {
        let ifont = FontId::new(14.0_f32, egui::FontFamily::Name("phosphor-regular".into()));
        ui.label(
            egui::RichText::new(icon_str)
                .font(ifont)
                .color(colors::APRICOT_DARK),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(title)
                .font(typo::doc_subtitle())
                .color(colors::TEXT_PRIMARY),
        );
    });
}

/// Rounded badge icon box (used for file cards and section emblems)
pub fn badge_icon(
    ui: &mut Ui,
    icon_str: &str,
    icon_size: f32,
    badge_size: f32,
    fill: Color32,
    stroke_col: Color32,
    icon_col: Color32,
) {
    let (box_rect, _) = ui.allocate_exact_size(Vec2::splat(badge_size), Sense::hover());
    ui.painter().rect_filled(box_rect, CornerRadius::same(6), fill);
    ui.painter().rect_stroke(
        box_rect,
        CornerRadius::same(6),
        Stroke::new(1.0_f32, stroke_col),
        egui::StrokeKind::Inside,
    );
    let ifont = FontId::new(icon_size, egui::FontFamily::Name("phosphor-regular".into()));
    ui.painter().text(
        box_rect.center(),
        egui::Align2::CENTER_CENTER,
        icon_str,
        ifont,
        icon_col,
    );
}
