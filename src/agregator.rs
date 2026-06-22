use std::{
    collections::{BTreeSet, HashSet},
    ops::Div,
};

use egui::{Button, Ui};
#[derive(PartialEq, Debug)]
pub enum Status {
    None,
    Remove,
}
#[derive(Debug)]
pub struct Agregator {
    status: Status,
    selected_items: BTreeSet<String>,
    items: BTreeSet<String>,
}
impl Agregator {
    pub fn default() -> Self {
        Self {
            status: Status::None,
            selected_items: Default::default(),
            items: Default::default(),
        }
    }
    fn draw_multiselect(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .max_height(180.0)
            .auto_shrink([false, true])
            .max_width(ui.available_width())
            .show(ui, |ui| {
                for item in &self.items {
                    let is_selected = self.selected_items.contains(item);
                    let label = &self.items.get(item).unwrap();
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui.selectable_label(is_selected, *label).clicked() {
                                if is_selected {
                                    self.selected_items.remove(item);
                                } else {
                                    self.selected_items.insert(item.to_string());
                                }
                            }
                        },
                    );
                }
            });
    }
    fn draw_button(&mut self, ui: &mut Ui) {
        self.status = match ui
            .add_sized([ui.available_width(), 30.0], Button::new("Удалить"))
            .clicked()
        {
            true => Status::Remove,
            false => Status::None,
        };
    }
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                self.draw_multiselect(ui);
                self.draw_button(ui);
            });
        });
    }
    pub fn multiselect(&mut self, set: &BTreeSet<String>) {
        self.items = set.clone();
    }
    pub fn selected(&self) -> &BTreeSet<String> {
        &self.selected_items
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
}
