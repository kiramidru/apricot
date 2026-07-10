use egui::{Button, CornerRadius, Response, Ui, Widget, WidgetText};

pub struct CustomButton {
    text: WidgetText,
<<<<<<< HEAD
    show_border: bool, // New field
=======
    show_border: bool,
>>>>>>> 40d0dee (init commit)
}

impl CustomButton {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            show_border: true,
        }
    }

    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }
}

impl Widget for CustomButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut btn = Button::new(self.text)
            .fill(egui::Color32::TRANSPARENT)
            .corner_radius(CornerRadius::same(8))
            .min_size(egui::vec2(32.0, 32.0));

        if !self.show_border {
            btn = btn.stroke(egui::Stroke::NONE);
        } else {
<<<<<<< HEAD
            btn = btn.stroke(egui::Stroke::new(1.2, egui::Color32::from_rgb(200,200,200)));
=======
            btn = btn.stroke(egui::Stroke::new(
                1.2,
                egui::Color32::from_rgb(200, 200, 200),
            ));
>>>>>>> 40d0dee (init commit)
        }

        ui.add(btn)
    }
}
<<<<<<< HEAD

=======
>>>>>>> 40d0dee (init commit)
