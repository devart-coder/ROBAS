use std::{
    fs::File,
    io::{BufReader, Error},
};

use calamine::{Reader, SheetVisible, Sheets, open_workbook_auto};
pub struct Document {
    sheets: Vec<String>,
    workbook: Option<Sheets<BufReader<File>>>,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            sheets: Default::default(),
            workbook: None,
        }
    }
}
impl Document {
    pub fn open(path_str: &str) -> Self {
        let file =
            open_workbook_auto(path_str).expect(&format!("Can't open document: {} ", path_str));
        Self {
            workbook: Some(file),
            sheets: Vec::default(),
        }
    }
    pub fn sheets(&mut self, visible: SheetVisible) -> &Vec<String> {
        if let Some(v) = &self.workbook {
            for sheet in v.sheets_metadata() {
                if sheet.visible == visible {
                    self.sheets.push(sheet.name.to_string());
                }
            }
        }
        &self.sheets
    }
}
