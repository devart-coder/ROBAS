use std::{
    collections::{BTreeSet, HashSet},
    ops::{Add, Mul, Sub},
    sync::atomic::{AtomicBool, Ordering},
};

use calamine::{Reader, SheetVisible};
use egui::{Rangef, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{
    agregator::{self, Agregator, Status},
    document::Document,
    sheets_search::Sheets,
};

#[derive(Default)]
pub struct Application {
    selected: String,
    file_opened: bool,
    workbook: Document,
    agregators: Vec<Agregator>,
    search_by: BTreeSet<String>,
    sheets: Sheets,
}
impl Application {
    fn create_head(&mut self, ui: &mut Ui) {
        egui::Panel::top("head")
            .default_size(60.0)
            .show_inside(ui, |ui| {
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
            });
    }
    fn create_table(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let mut table =
                TableBuilder::new(ui).column(Column::auto().at_least(100.0).resizable(true));
            for _ in &self.agregators {
                table = table.column(Column::auto().resizable(true).at_least(100.0));
            }
            table
                .striped(true)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.strong(self.selected.to_string());
                    });
                    for agregator in &self.agregators {
                        header.col(|ui| {
                            for text in agregator.selected() {
                                ui.strong(text);
                            }
                        });
                    }
                })
                .body(|mut body| {
                    body.row(50.0, |mut row| {
                        let draw_cell = |ui: &mut egui::Ui, text: String| {
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
                            ui.add_space(4.0);
                            ui.label(text);
                        };
                        for agregator in &self.agregators {
                            row.col(|ui| {
                                let text =
                                    agregator.selected().iter().map(|r| r.to_string()).collect();
                                draw_cell(ui, text);
                            });
                        }
                    });
                });
        });
    }
    fn create_menu(&mut self, ui: &mut Ui) {
        egui::Panel::top("menu")
            .default_size(30.0)
            .show_inside(ui, |ui| {
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
            });
    }
    fn create_agregators_column(&mut self, ui: &mut Ui) {
        let width = ui.available_width().mul(0.2);
        egui::Panel::left("agregators")
            .default_size(width)
            .size_range(Rangef::new(width.sub(100.0), width.add(100.0)))
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
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
                });
            });
    }
    fn create_sheets(&mut self, ui: &mut Ui) {
        self.sheets
            .show_sheets(self.workbook.sheets(SheetVisible::Visible));
        self.sheets.draw(ui);
    }
}
impl eframe::App for Application {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.create_menu(ui);
        if self.file_opened == true {
            self.create_head(ui);
            self.create_agregators_column(ui);
            self.create_sheets(ui);
            self.create_table(ui);
            if let Some(v) = self.sheets.statement() {
                println!("Selected: {:?}", v);
                let map = self
                    .workbook
                    .action()
                    .word(&self.selected)
                    .with(&self.agregators)
                    .in_sheets(v)
                    .search();
                println!("Map: {:#?}", map);
            }
        }
    }
}
