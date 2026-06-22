use std::{
    collections::HashSet,
    sync::atomic::{AtomicBool, Ordering},
};

use calamine::{Reader, SheetVisible};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{
    agregator::{self, Agregator, Status},
    document::Document,
};

#[derive(Default)]
pub struct Application {
    selected: String,
    file_opened: bool,
    workbook: Document,
    agregators: Vec<Agregator>,
    search_by: HashSet<String>,
}
impl Application {
    fn create_head(&mut self, ui: &mut Ui) {
        ui.heading("Настройки");
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.heading("Искать");
                let _ = egui::ComboBox::from_id_salt("selected")
                    .selected_text(&self.selected)
                    .show_ui(ui, |ui| {
                        for item in &self.search_by {
                            ui.selectable_value(
                                &mut self.selected,
                                item.to_string(),
                                item.to_string(),
                            );
                        }
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
        egui::ScrollArea::both().show(ui, |ui| {
            TableBuilder::new(ui)
                .column(Column::auto().resizable(true))
                .striped(true)
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong(self.selected.to_string());
                    });
                    header.col(|ui| {
                        for agregator in &self.agregators {
                            for text in agregator.selected() {
                                ui.strong(text);
                            }
                        }
                    });
                })
                .body(|mut body| {
                    body.row(50.0, |mut row| {
                        let draw_cell = |ui: &mut egui::Ui, text: String| {
                            // 1. Рисуем рамку вокруг ячейки (эффект Excel-сетки)
                            let stroke = egui::Stroke::new(
                                1.0,
                                ui.visuals().widgets.noninteractive.bg_stroke.color,
                            );
                            ui.painter().rect_stroke(
                                ui.max_rect(),
                                0.0,
                                stroke,
                                egui::StrokeKind::Inside,
                            );

                            // 2. Добавляем внутренний отступ, чтобы текст не прилипал к линиям
                            ui.add_space(4.0);
                            ui.label(text);
                        };
                        row.col(|ui| {
                            draw_cell(ui, self.selected.clone());
                        });
                        row.col(|ui| {
                            draw_cell(ui, self.selected.clone());
                        });
                    });
                });
        });
    }
    fn create_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Файл", |ui| {
            if ui.button("📂Открыть").clicked() {
                let file_dialog = rfd::FileDialog::new()
                    .add_filter("Exel files", &["xlsx", "xlsm", "xls", "xlsb"]);
                if let Some(file_path) = file_dialog.pick_file() {
                    if let Some(path_str) = file_path.to_str() {
                        self.workbook = Document::open(path_str);
                        self.search_by = self.workbook.search_pos("№ п/п").clone();
                        self.file_opened = true;
                    }
                }
            }
        });
    }
    fn create_agregators_column(&mut self, ui: &mut Ui) {
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
                                    let mut clone = self.search_by.clone();
                                    clone.remove(&self.selected);
                                    item.multiselect(&clone);
                                    item.draw(ui);
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
impl eframe::App for Application {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.create_menu(ui);
        if self.file_opened == true {
            self.create_head(ui);
            self.create_agregators_column(ui);
        }
    }
}
