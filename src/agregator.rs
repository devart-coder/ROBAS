use std::{collections::HashSet, ops::Deref};

use egui::{Button, Response, Ui, Widget};
#[derive(PartialEq, Debug)]
pub enum Status {
    None,
    Remove,
}
// #[derive(Default, Debug)]
pub struct Agregator {
    status: Status,
    selected_items: HashSet<usize>,
    items: Vec<String>,
}
impl Agregator {
    pub fn default() -> Self {
        Self {
            status: Status::None,
            selected_items: HashSet::new(),
            items: vec!["First".into(), "Second".into(), "Third".into()],
        }
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let panel_width = 200.0;
            ui.group(|ui| {
                let g = egui::ScrollArea::vertical()
                    .max_height(180.0)
                    .auto_shrink([false, true])
                    .max_width(panel_width)
                    .show(ui, |ui| {
                        for i in 0..self.items.len() {
                            let is_selected = self.selected_items.contains(&i);
                            let label = &self.items[i];
                            if ui.selectable_label(is_selected, label).clicked() {
                                if is_selected {
                                    self.selected_items.remove(&i);
                                } else {
                                    self.selected_items.insert(i);
                                }
                            }
                        }
                    });
                self.status = match ui
                    .add_sized([panel_width, 30.0], Button::new("Удалить"))
                    .clicked()
                {
                    true => Status::Remove,
                    false => Status::None,
                };
            });
        });
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
}
