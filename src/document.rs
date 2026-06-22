use std::{collections::HashSet, fs::File, io::BufReader};

use calamine::{Reader, SheetVisible, Sheets, open_workbook_auto};
pub struct Document {
    sheets: Vec<String>,
    workbook: Option<Sheets<BufReader<File>>>,
    search_by_list: HashSet<String>,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            sheets: Default::default(),
            workbook: None,
            search_by_list: HashSet::default(),
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
            search_by_list: HashSet::default(),
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
    pub fn search_pos(&mut self, value: &str) -> &HashSet<String> {
        if let Some(v) = &mut self.workbook {
            for sheets in v.worksheets() {
                let result = sheets
                    .1
                    .rows()
                    .position(|row| row.iter().any(|cell| value == cell.to_string().trim()));
                if let Some(r) = result {
                    for s in sheets.1.rows().nth(r).unwrap().to_vec().iter().skip(1) {
                        self.search_by_list.insert(s.to_string().trim().to_string());
                    }
                }
            }
        }
        &self.search_by_list
    }
}
