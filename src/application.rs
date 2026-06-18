use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct Application {
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
                let appeted_button = ui.button(egui::RichText::new("Добавить").heading());
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
        self.create_table(ui);
    }
}
