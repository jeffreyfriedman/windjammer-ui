#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
use super::table::{Table, TableColumn, TableRow};
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
        DataTable { table: Table::new(), empty_message: String::from("No rows"), caption: String::new(), scrollable: false }
}
#[inline]
pub fn caption(mut self, caption: impl Into<String>) -> DataTable {
        self.caption = caption.into();
        self
}
#[inline]
pub fn empty_message(mut self, message: impl Into<String>) -> DataTable {
        self.empty_message = message.into();
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
        let mut caption_html = String::new();
        if !self.caption.is_empty() {
            caption_html = format!("<div class='wj-datatable-caption'>{}</div>", self.caption);
        }
        let table_block = {
            if self.scrollable {
                format!("<div class='lk-table-scroll'>{}</div>", body)
            } else {
                body.clone()
            }
        };
        format!("<div class='wj-datatable' data-empty-message='{}'>{}{}</div>", self.empty_message, caption_html, table_block)
}
}

