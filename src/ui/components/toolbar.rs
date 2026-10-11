use super::widgets::{icon_btn, page_box, zoom_dropdown};
use crate::ui::theme::{colors, typo};
use eframe::egui::{self, Pos2, Rect, Ui};
use egui_phosphor_icons::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarAction {
    None,
    PrevPage,
    NextPage,
    ZoomIn,
    ZoomOut,
}

pub fn render_toolbar(
    ui: &mut Ui,
    rect: Rect,
    current_page: usize,
    total_pages: usize,
    zoom_pct: usize,
) -> ToolbarAction {
    let mut action = ToolbarAction::None;
    let mid_y = rect.center().y;

    // --- Left: Search, Page Up, Page [1] / 13, Page Down ---
    let mut lx = rect.min.x + 16.0;

    // Search icon
    let search_rect = Rect::from_center_size(Pos2::new(lx + 10.0, mid_y), egui::vec2(22.0, 22.0));
    let mut search_ui = ui.new_child(egui::UiBuilder::new().max_rect(search_rect));
    icon_btn(&mut search_ui, icons::MAGNIFYING_GLASS, 13.0, 22.0);
    lx += 26.0;

    // Caret Up (page up)
    let up_rect = Rect::from_center_size(Pos2::new(lx + 9.0, mid_y), egui::vec2(22.0, 22.0));
    let mut up_ui = ui.new_child(egui::UiBuilder::new().max_rect(up_rect));
    if icon_btn(&mut up_ui, icons::CARET_UP, 11.0, 22.0).clicked() {
        action = ToolbarAction::PrevPage;
    }
    lx += 24.0;

    // Page box: [ 1 ]
    let box_rect = Rect::from_center_size(Pos2::new(lx + 11.0, mid_y), egui::vec2(22.0, 22.0));
    let mut box_ui = ui.new_child(egui::UiBuilder::new().max_rect(box_rect));
    page_box(&mut box_ui, current_page);
    lx += 28.0;

    // Page count label: `/ 13`
    let total_str = format!("/ {}", total_pages);
    let tr = ui.painter().text(
        Pos2::new(lx, mid_y),
        egui::Align2::LEFT_CENTER,
        total_str,
        typo::toolbar_text(),
        colors::TEXT_MUTED,
    );
    lx += tr.width() + 6.0;

    // Caret Down (page down)
    let down_rect = Rect::from_center_size(Pos2::new(lx + 9.0, mid_y), egui::vec2(22.0, 22.0));
    let mut down_ui = ui.new_child(egui::UiBuilder::new().max_rect(down_rect));
    if icon_btn(&mut down_ui, icons::CARET_DOWN, 11.0, 22.0).clicked() {
        action = ToolbarAction::NextPage;
    }

    // --- Center: Minus, Dropdown 100% ▾, Plus ---
    let cx = rect.center().x;

    // Minus
    let minus_rect = Rect::from_center_size(Pos2::new(cx - 44.0, mid_y), egui::vec2(22.0, 22.0));
    let mut minus_ui = ui.new_child(egui::UiBuilder::new().max_rect(minus_rect));
    if icon_btn(&mut minus_ui, icons::MINUS, 12.0, 22.0).clicked() {
        action = ToolbarAction::ZoomOut;
    }

    // Dropdown pill
    let zoom_rect = Rect::from_center_size(Pos2::new(cx, mid_y), egui::vec2(56.0, 22.0));
    let mut zoom_ui = ui.new_child(egui::UiBuilder::new().max_rect(zoom_rect));
    zoom_dropdown(&mut zoom_ui, zoom_pct);

    // Plus
    let plus_rect = Rect::from_center_size(Pos2::new(cx + 44.0, mid_y), egui::vec2(22.0, 22.0));
    let mut plus_ui = ui.new_child(egui::UiBuilder::new().max_rect(plus_rect));
    if icon_btn(&mut plus_ui, icons::PLUS, 12.0, 22.0).clicked() {
        action = ToolbarAction::ZoomIn;
    }

    // --- Right: Fullscreen, Print, Dots ---
    let mut rx = rect.max.x - 16.0;

    // Three dots
    let dots_rect = Rect::from_center_size(Pos2::new(rx - 10.0, mid_y), egui::vec2(22.0, 22.0));
    let mut dots_ui = ui.new_child(egui::UiBuilder::new().max_rect(dots_rect));
    icon_btn(&mut dots_ui, icons::DOTS_THREE, 13.0, 22.0);
    rx -= 26.0;

    // Print
    let print_rect = Rect::from_center_size(Pos2::new(rx - 10.0, mid_y), egui::vec2(22.0, 22.0));
    let mut print_ui = ui.new_child(egui::UiBuilder::new().max_rect(print_rect));
    icon_btn(&mut print_ui, icons::PRINTER, 13.0, 22.0);
    rx -= 26.0;

    // Fullscreen / expand arrows
    let fs_rect = Rect::from_center_size(Pos2::new(rx - 10.0, mid_y), egui::vec2(22.0, 22.0));
    let mut fs_ui = ui.new_child(egui::UiBuilder::new().max_rect(fs_rect));
    icon_btn(&mut fs_ui, icons::ARROWS_OUT_SIMPLE, 12.5, 22.0);

    action
}
