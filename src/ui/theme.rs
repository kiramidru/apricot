#![allow(dead_code)]
use eframe::egui::{self, Color32, FontFamily, FontId};
use std::sync::Arc;

pub mod colors {
    use super::Color32;

    pub const WINDOW_BG: Color32 = Color32::from_rgb(0xF8, 0xF6, 0xF4);
    pub const CARD_BG: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
    pub const CARD_BORDER: Color32 = Color32::from_rgb(0xEB, 0xE7, 0xE3);
    pub const CARD_SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 8);

    pub const DIVIDER: Color32 = Color32::from_rgb(0xF2, 0xEE, 0xEB);

    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0x1C, 0x1A, 0x18);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x7A, 0x73, 0x6E);
    pub const TEXT_SUBTLE: Color32 = Color32::from_rgb(0x9E, 0x97, 0x92);
    pub const BREADCRUMB_CHEVRON: Color32 = Color32::from_rgb(0xC4, 0xBD, 0xB8);

    pub const ICON_BTN: Color32 = Color32::from_rgb(0x61, 0x5B, 0x56);
    pub const ICON_BTN_HOVER: Color32 = Color32::from_rgb(0x1C, 0x1A, 0x18);
    pub const BTN_BORDER: Color32 = Color32::from_rgb(0xDE, 0xD8, 0xD2);
    pub const BTN_HOVER_BG: Color32 = Color32::from_rgb(0xF9, 0xF6, 0xF4);

    pub const PAGE_BOX_BG: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
    pub const PAGE_BOX_BORDER: Color32 = Color32::from_rgb(0xDE, 0xD8, 0xD2);

    // --- Apricot Brand Palette ---
    pub const APRICOT_PRIMARY: Color32 = Color32::from_rgb(0xE0, 0x6D, 0x38);
    pub const APRICOT_DARK: Color32 = Color32::from_rgb(0xB3, 0x4E, 0x1B);
    pub const APRICOT_SOFT: Color32 = Color32::from_rgb(0xFD, 0xF2, 0xEB);
    pub const APRICOT_BORDER: Color32 = Color32::from_rgb(0xF5, 0xC8, 0xB0);
    pub const APRICOT_HOVER: Color32 = Color32::from_rgb(0xFA, 0x7E, 0x48);

    pub const DOC_TITLE: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);
    pub const DOC_SUBTITLE: Color32 = Color32::from_rgb(0x14, 0x12, 0x10);
    pub const DOC_LEGISLATION: Color32 = Color32::from_rgb(0x28, 0x24, 0x22);
    pub const DOC_RULE: Color32 = Color32::from_rgb(0xCE, 0xC8, 0xC4);
    pub const DOC_BODY: Color32 = Color32::from_rgb(0x1E, 0x1C, 0x1A);
}

pub const FONT_ROBOTO_MEDIUM: &str = "roboto-medium";
pub const FONT_ROBOTO_BOLD: &str = "roboto-bold";
pub const FONT_DEJAVU_SANS: &str = "dejavu-sans";
pub const FONT_DEJAVU_SANS_BOLD: &str = "dejavu-sans-bold";
pub const FONT_DEJAVU_SANS_OBLIQUE: &str = "dejavu-sans-oblique";
pub const FONT_LIBERATION_BOLD: &str = "liberation-bold";

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    egui_phosphor_icons::add_fonts(&mut fonts);

    fonts.font_data.insert(
        "roboto-regular".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Roboto-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "roboto-medium".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Roboto-Medium.ttf"
        ))),
    );
    fonts.font_data.insert(
        "roboto-bold".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Roboto-Bold.ttf"
        ))),
    );
    fonts.font_data.insert(
        "dejavu-sans".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/DejaVuSans.ttf"
        ))),
    );
    fonts.font_data.insert(
        "dejavu-sans-bold".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/DejaVuSans-Bold.ttf"
        ))),
    );
    fonts.font_data.insert(
        "dejavu-sans-oblique".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/DejaVuSans-Oblique.ttf"
        ))),
    );
    fonts.font_data.insert(
        "liberation-bold".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/LiberationSans-Bold.ttf"
        ))),
    );

    let prop = fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default();
    prop.insert(0, "phosphor-icons".to_owned());
    prop.insert(0, "roboto-regular".to_owned());

    fonts.families.insert(
        FontFamily::Name(FONT_ROBOTO_MEDIUM.into()),
        vec!["roboto-medium".to_owned(), "phosphor-icons".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_ROBOTO_BOLD.into()),
        vec!["roboto-bold".to_owned(), "phosphor-icons".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_DEJAVU_SANS.into()),
        vec!["dejavu-sans".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_DEJAVU_SANS_BOLD.into()),
        vec!["dejavu-sans-bold".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_DEJAVU_SANS_OBLIQUE.into()),
        vec!["dejavu-sans-oblique".to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_LIBERATION_BOLD.into()),
        vec!["liberation-bold".to_owned()],
    );

    ctx.set_fonts(fonts);
}

pub fn configure_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(colors::TEXT_PRIMARY);
    visuals.panel_fill = colors::WINDOW_BG;
    visuals.window_fill = colors::WINDOW_BG;
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 5.0);
    ctx.set_style(style);
}

pub mod typo {
    use super::*;

    pub fn breadcrumb() -> FontId {
        FontId::new(11.0, FontFamily::Proportional)
    }

    pub fn breadcrumb_active() -> FontId {
        FontId::new(11.0, FontFamily::Name(FONT_ROBOTO_MEDIUM.into()))
    }

    pub fn toolbar_text() -> FontId {
        FontId::new(11.5, FontFamily::Proportional)
    }

    pub fn doc_title() -> FontId {
        FontId::new(17.0, FontFamily::Name(FONT_LIBERATION_BOLD.into()))
    }

    pub fn doc_subtitle() -> FontId {
        FontId::new(13.0, FontFamily::Name(FONT_ROBOTO_BOLD.into()))
    }

    pub fn doc_legislation() -> FontId {
        FontId::new(11.0, FontFamily::Name(FONT_DEJAVU_SANS_OBLIQUE.into()))
    }

    pub fn doc_body() -> FontId {
        FontId::new(10.5, FontFamily::Name(FONT_DEJAVU_SANS.into()))
    }

    pub fn doc_body_bold() -> FontId {
        FontId::new(10.5, FontFamily::Name(FONT_DEJAVU_SANS_BOLD.into()))
    }
}
