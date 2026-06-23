use std::{
    collections::{BTreeSet, HashSet},
    ops::{Div, Mul, Sub},
};

use egui::{Rangef, Scene, Sense, Ui};
#[derive(Default)]
pub struct Sheets {
    sheets_search_names: BTreeSet<String>,
    selected: BTreeSet<String>,
}
impl Sheets {
    pub fn draw(&mut self, ui: &mut Ui) {
        let width = ui.available_width().mul(0.2);
        egui::Panel::right("right_panel")
            .resizable(true) // Можно ли пользователю менять её ширину мышкой
            .size_range(Rangef::new(width.sub(100.0), width.sub(100.0)))
            .size_range((width * 0.2)..=(width * 0.2)) // Выделяем ровно 20%
            .default_size(width)
            .show_inside(ui, |ui| {
                ui.label("Листы");
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [width.sub(30.0).div(2.0).sub(5.0), 30.0],
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
                                [width.sub(30.0).div(2.0).sub(5.0), 30.0],
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
                            .max_width(width)
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
                    ui.add_sized([width.sub(30.0), 30.0], egui::Button::new("Найти"));
                });
            });
    }
    pub fn search_by(&self) {}
    pub fn show_sheets(&mut self, sheets: &BTreeSet<String>) {
        self.sheets_search_names = sheets.clone();
    }
}
