use std::fmt::format;

use egui::{Response, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::agregator::{Agregator, Status};

#[derive(Default)]
pub struct Application {
    agregators: Vec<Agregator>,
    search_by: RichText,
}
impl Application {
    fn create_top(&mut self, ui: &mut Ui) {
        ui.heading("Настройки");
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.heading("Искать");
                let _ = egui::ComboBox::from_id_salt("selected")
                    .selected_text(format!("{}", self.search_by.text()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.search_by,
                            RichText::from("First").heading(),
                            RichText::from("First").heading(),
                        );
                    });
                let _ = ui.heading("Агрегатор");
                if ui.button("Добавить").clicked() {
                    let ag = Agregator::default();
                    self.agregators.push(ag);
                }
            });
        });
    }
    fn create_table(&mut self, ui: &mut Ui) {
        TableBuilder::new(ui)
            .column(Column::auto().resizable(true))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("First column");
                });
                header.col(|ui| {
                    ui.heading("Second column");
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Hello");
                    });
                    row.col(|ui| {
                        ui.button("world!");
                    });
                });
            });
    }
}
impl eframe::App for Application {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.create_top(ui);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                if self.agregators.is_empty() {
                } else {
                    let mut to_remove = usize::MAX;
                    for (index, item) in &mut self.agregators.iter_mut().enumerate() {
                        match item.status() {
                            Status::Remove => {
                                to_remove = index;
                            }
                            Status::None => {
                                ui.push_id(index, |ui| {
                                    ui.label(format!("Агрегатор {}", index));
                                    item.ui(ui);
                                });
                            }
                        }
                    }
                    if to_remove != usize::MAX {
                        self.agregators.remove(to_remove);
                    }
                }
            });
            self.create_table(ui);
        });
    }
}
