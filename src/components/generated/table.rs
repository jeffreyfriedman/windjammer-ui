#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct TableColumn {
    pub header: String,
    pub width: String,
}

impl TableColumn {
    #[inline]
    pub fn new(header: String) -> TableColumn {
        TableColumn {
            header,
            width: "auto".to_string(),
        }
    }
    #[inline]
    pub fn width(mut self, width: String) -> TableColumn {
        self.width = width;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct TableRow {
    pub cells: Vec<String>,
}

impl TableRow {
    #[inline]
    pub fn new() -> TableRow {
        TableRow { cells: Vec::new() }
    }
    #[inline]
    pub fn cell(mut self, content: String) -> TableRow {
        self.cells.push(content);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct Table {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub striped: bool,
    pub bordered: bool,
    pub hoverable: bool,
}

impl Table {
    #[inline]
    pub fn new() -> Table {
        Table {
            columns: Vec::new(),
            rows: Vec::new(),
            striped: true,
            bordered: true,
            hoverable: true,
        }
    }
    #[inline]
    pub fn column(mut self, col: TableColumn) -> Table {
        self.columns.push(col);
        self
    }
    #[inline]
    pub fn row(mut self, row: TableRow) -> Table {
        self.rows.push(row);
        self
    }
    #[inline]
    pub fn striped(mut self, striped: bool) -> Table {
        self.striped = striped;
        self
    }
    #[inline]
    pub fn bordered(mut self, bordered: bool) -> Table {
        self.bordered = bordered;
        self
    }
    #[inline]
    pub fn hoverable(mut self, hoverable: bool) -> Table {
        self.hoverable = hoverable;
        self
    }
}

impl Renderable for Table {
    #[inline]
    fn render(&self) -> String {
        let mut classes = String::from("wj-table");
        if self.striped {
            classes.push_str(" wj-table-striped");
        }
        if self.bordered {
            classes.push_str(" wj-table-bordered");
        }
        if self.hoverable {
            classes.push_str(" wj-table-hover");
        }
        let mut html = format!("<table class='{}'>", classes);
        html.push_str("<thead><tr>");
        for col in &self.columns {
            if col.width == "auto" {
                html.push_str("<th>");
            } else {
                html.push_str(&format!("<th style='width: {}'>", col.width));
            }
            html.push_str(&col.header);
            html.push_str("</th>");
        }
        html.push_str("</tr></thead><tbody>");
        for row in &self.rows {
            html.push_str("<tr>");
            for cell in &row.cells {
                html.push_str("<td>");
                html.push_str(cell);
                html.push_str("</td>");
            }
            html.push_str("</tr>");
        }
        html.push_str("</tbody></table>");
        html
    }
}
