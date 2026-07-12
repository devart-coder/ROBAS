use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs::File,
    io::BufReader,
    ops::Deref,
};

use calamine::{Data, DataType, Range, Reader, SheetVisible, Sheets, open_workbook_auto};

use crate::agregator::Agregator;
// use egui::FontSelection::Default;
pub struct Document {
    sheets: BTreeSet<String>,
    workbook: Option<Sheets<BufReader<File>>>,
    search_by_list: BTreeSet<String>,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            sheets: Default::default(),
            workbook: None,
            search_by_list: Default::default(),
        }
    }
}
impl Document {
    pub fn open(path_str: &str) -> Self {
        let file =
            open_workbook_auto(path_str).expect(&format!("Can't open document: {} ", path_str));
        Self {
            workbook: Some(file),
            sheets: Default::default(),
            search_by_list: Default::default(),
        }
    }
    pub fn sheets(&mut self, visible: SheetVisible) -> &BTreeSet<String> {
        if let Some(v) = &self.workbook {
            for sheet in v.sheets_metadata() {
                if sheet.visible == visible {
                    self.sheets.insert(sheet.name.to_string());
                }
            }
        }
        &self.sheets
    }
    pub fn search_pos(&mut self, value: &str) -> &BTreeSet<String> {
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
    pub fn action(&mut self) -> SearchAction<'_> {
        SearchAction::new(&mut self.workbook)
    }
}
// #[derive(Default)]
pub struct SearchAction<'a> {
    word: String,
    agregators: Vec<Agregator>,
    sheets: BTreeSet<String>,
    workboork: &'a mut Option<Sheets<BufReader<File>>>,
}
impl<'a> SearchAction<'a> {
    pub fn new(w: &'a mut Option<Sheets<BufReader<File>>>) -> Self {
        Self {
            workboork: w,
            agregators: Vec::<Agregator>::new(),
            word: Default::default(),
            sheets: Default::default(),
        }
    }
    pub fn word(mut self, w: &str) -> Self {
        self.word = w.to_string();
        self
    }
    pub fn with(mut self, v: &Vec<Agregator>) -> Self {
        for agregator in v {
            self.agregators.push((*agregator).clone());
        }
        self
    }
    pub fn in_sheets(mut self, v: &BTreeSet<String>) -> Self {
        self.sheets = v.clone();
        self
    }
    pub fn search(&mut self) -> BTreeMap<String, Vec<i32>> {
        let mut map = BTreeMap::new();
        if let Some(workbook) = &mut self.workboork {
            for (name, sheet) in workbook.worksheets() {
                if self.sheets.contains(&name) {
                    if let Some((row, column)) =
                        self.find_word_position(&self.word.trim().to_string(), &sheet)
                    {
                        for h in row..sheet.height() {
                            let r = sheet.rows().nth(h).unwrap().to_vec();
                            let r = r.get(column).unwrap();
                            if !r.is_empty() && !r.to_string().contains("Итого") {
                                for a in self.agregators{
                                    
                                }
                                map.insert(r.to_string().trim().to_string(), vec![]);
                            }
                        }
                    }
                }
            }
        }
        map
    }
    fn find_word_position(&self, word: &String, range: &Range<Data>) -> Option<(usize, usize)> {
        for (row_id, row) in range.rows().enumerate() {
            match row.iter().position(|cell| cell.to_string() == *word) {
                Some(column_id) => return Some((row_id, column_id)),
                None => None::<(usize, usize)>,
            };
        }
        None
    }
    // fn find_and_action<T>(&self, word: &String, range: &Range<Data>, p: T)
    // where
    // T: FnOnce(BTreeMap<String, Vec<i32>>),
    // {

    // }
}
// search_by(string).in_sheets(BTreeSet)->Map<String,Vec<i32>>
