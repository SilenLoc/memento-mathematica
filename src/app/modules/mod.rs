use crate::app::memath::{addition, subtraction};
use bigdecimal::BigDecimal;
use egui::Ui;

#[derive(Default)]
pub struct SubtractionState {
    result: String,
    left: i32,
    right: i32,
}

impl SubtractionState {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.left, 0..=100));
        ui.add(egui::Slider::new(&mut self.right, 0..=100));

        if ui.button("subtract").clicked() {
            self.result =
                subtraction(BigDecimal::from(self.left), BigDecimal::from(self.right)).to_string()
        }

        ui.label(format!("Result {}", &self.result));
    }
}

#[derive(Default)]
pub struct AdditionState {
    result: String,
    left: i32,
    right: i32,
}

impl AdditionState {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.left, 0..=100));
        ui.add(egui::Slider::new(&mut self.right, 0..=100));

        if ui.button("add").clicked() {
            self.result =
                addition(BigDecimal::from(self.left), BigDecimal::from(self.right)).to_string()
        }

        ui.label(format!("Result {}", &self.result));
    }
}
