use std::fmt::format;

use calamine::{Reader, SheetVisible, Xls, open_workbook, open_workbook_auto};
use egui::{Response, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{
    agregator::{Agregator, Status},
    document::Document,
};

#[derive(Default)]
pub struct Application {
    workbook: Document,
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

    fn create_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Файл", |ui| {
            if ui.button("📂Открыть").clicked() {
                let file_dialog = rfd::FileDialog::new()
                    .add_filter("Exel files", &["xlsx", "xlsm", "xls", "xlsb"]);
                if let Some(file_path) = file_dialog.pick_file() {
                    if let Some(path_str) = file_path.to_str() {
                        #[cfg(debug_assertions)]
                        {
                            println!("Path_str: {}", path_str);
                        }
                        self.workbook = Document::open(path_str);
                        // let excel = open_workbook_auto(file_path).unwrap();
                        // #[cfg(debug_assertions)]
                        // {
                        // println!("Document was opened.",);
                        // println!("Sheets range: {:?}", excel.sheet_names());
                        // }
                        #[cfg(debug_assertions)]
                        {
                            println!(
                                "Visible sheets range: {:?}",
                                self.workbook.sheets(SheetVisible::Visible)
                            );
                        }
                        self.workbook.search_pos("№ п/п");
                    }
                }
                // open_workbook("")
            };
        });
    }
}
impl eframe::App for Application {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.create_menu(ui);
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
