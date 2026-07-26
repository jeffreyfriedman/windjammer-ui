#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::table::{Table, TableColumn, TableRow};
use super::traits::Renderable;
#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataTable {
    pub table: Table,
    pub empty_message: String,
    pub caption: String,
    pub scrollable: bool,
}

impl DataTable {
    #[inline]
    pub fn new() -> DataTable {
        DataTable {
            table: Table::new(),
            empty_message: "No rows".to_string(),
            caption: "".to_string(),
            scrollable: false,
        }
    }
    #[inline]
    pub fn caption(mut self, caption: String) -> DataTable {
        self.caption = caption;
        self
    }
    #[inline]
    pub fn empty_message(mut self, message: String) -> DataTable {
        self.empty_message = message;
        self
    }
    #[inline]
    pub fn scrollable(mut self, on: bool) -> DataTable {
        self.scrollable = on;
        self
    }
    #[inline]
    pub fn column(mut self, col: TableColumn) -> DataTable {
        self.table = self.table.column(col);
        self
    }
    #[inline]
    pub fn row(mut self, row: TableRow) -> DataTable {
        self.table = self.table.row(row);
        self
    }
}

impl Renderable for DataTable {
    #[inline]
    fn render(&self) -> String {
        let body = self.table.render();
        let mut caption_html = "".to_string();
        if !self.caption.is_empty() {
            caption_html = format!(
                "<div class='wj-datatable-caption'>{}</div>",
                self.caption.clone()
            );
        }
        let table_block = if self.scrollable {
            format!("<div class='lk-table-scroll'>{}</div>", body)
        } else {
            body
        };
        format!(
            "<div class='wj-datatable' data-empty-message='{}'>{}{}</div>",
            self.empty_message.clone(),
            caption_html,
            table_block
        )
    }
}
