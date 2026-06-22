use std::{
    collections::{BTreeSet, HashSet},
    ops::{Div, Sub},
};

use egui::{Scene, Sense, Ui};
#[derive(Default)]
pub struct Sheets {
    sheets_search_names: BTreeSet<String>,
    selected: BTreeSet<String>,
}
impl Sheets {
    pub fn draw(&mut self, ui: &mut Ui) {
        let width = ui.available_width();
        egui::Panel::right("right_panel")
            .resizable(true) // Можно ли пользователю менять её ширину мышкой
            .default_size(width)
            .show_inside(ui, |ui| {
                ui.label("Листы");
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [width.div(2.0).sub(5.0), 30.0],
                                egui::Button::new("Выделить все"),
                            )
                            .clicked()
                        {
                            for name in &self.sheets_search_names {
                                self.selected.insert(name.to_owned());
                            }
                        };
                        if ui
                            .add_sized(
                                [width.div(2.0).sub(5.0), 30.0],
                                egui::Button::new("Очистить"),
                            )
                            .clicked()
                        {
                            self.selected.clear();
                        };
                    });
                    ui.group(|ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, true])
                            .max_width(ui.available_width())
                            .show(ui, |ui| {
                                for item in &self.sheets_search_names {
                                    let is_selected = self.selected.contains(item);
                                    let label = &self.sheets_search_names.get(item).unwrap();
                                    ui.with_layout(
                                        egui::Layout::top_down_justified(egui::Align::Center),
                                        |ui| {
                                            if ui.selectable_label(is_selected, *label).clicked() {
                                                if is_selected {
                                                    self.selected.remove(item);
                                                } else {
                                                    self.selected.insert(item.to_string());
                                                }
                                            }
                                        },
                                    );
                                }
                            });
                    });
                    ui.add_sized([width, 30.0], egui::Button::new("Найти"));
                });
            });
    }
    pub fn search_by(&self) {}
    pub fn show_sheets(&mut self, sheets: &BTreeSet<String>) {
        self.sheets_search_names = sheets.clone();
    }
}
