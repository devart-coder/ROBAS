use std::{collections::HashSet, ops::Deref};

use egui::{Button, Response, Ui, Widget};
#[derive(PartialEq, Debug)]
pub enum Status {
    None,
    Remove,
}
#[derive(Debug)]
pub struct Agregator {
    status: Status,
    selected_items: HashSet<usize>,
    items: Vec<String>,
    width: f32,
}
impl Agregator {
    pub fn default() -> Self {
        Self {
            status: Status::None,
            selected_items: HashSet::new(),
            items: vec!["First".into(), "Second".into(), "Third".into()],
            width: 200.0,
        }
    }
    fn draw_multiselect(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .max_height(180.0)
            .auto_shrink([false, true])
            .max_width(self.width)
            .show(ui, |ui| {
                for i in 0..self.items.len() {
                    let is_selected = self.selected_items.contains(&i);
                    let label = &self.items[i];
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui.selectable_label(is_selected, label).clicked() {
                                if is_selected {
                                    self.selected_items.remove(&i);
                                } else {
                                    self.selected_items.insert(i);
                                }
                            }
                        },
                    );
                }
            });
    }
    fn draw_button(&mut self, ui: &mut Ui) {
        self.status = match ui
            .add_sized([self.width, 30.0], Button::new("Удалить"))
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
    pub fn status(&self) -> &Status {
        &self.status
    }
}
