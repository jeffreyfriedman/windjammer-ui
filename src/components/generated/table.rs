#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

#[derive(Debug, Clone, Default)]
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
    fn render(self) -> String {
        let mut html = String::new();
        let border_style = {
            if self.bordered {
                "border: 1px solid #e2e8f0; border-collapse: collapse;".to_string()
            } else {
                "border-collapse: collapse;".to_string()
            }
        };
        html.push_str("<table style='width: 100%; ");
        html.push_str(&border_style);
        html.push_str("'>");
        html.push_str("<thead style='background: #f7fafc; border-bottom: 2px solid #e2e8f0;'>");
        html.push_str("<tr>");
        for col in &self.columns {
            html.push_str("<th style='padding: 12px; text-align: left; font-weight: 600; color: #2d3748; width: ");
            html.push_str(&col.width);
            if self.bordered {
                html.push_str("; border: 1px solid #e2e8f0;")
            }
            html.push_str("'>");
            html.push_str(&col.header.clone());
            html.push_str("</th>");
        }
        html.push_str("</tr>");
        html.push_str("</thead>");
        html.push_str("<tbody>");
        for (row_index, row) in self.rows.iter().enumerate() {
            let bg_color = {
                if self.striped && row_index % 2 == 1 {
                    "background: #f7fafc;".to_string()
                } else {
                    "background: white;".to_string()
                }
            };
            let hover_style = {
                if self.hoverable {
                    " onmouseover='this.style.background=\"#edf2f7\"' onmouseout='this.style.background=\"".to_string()
                } else {
                    "".to_string()
                }
            };
            html.push_str("<tr style='");
            html.push_str(&bg_color);
            html.push('\'');
            if self.hoverable {
                html.push_str(&hover_style);
                if self.striped && row_index % 2 == 1 {
                    html.push_str("#f7fafc")
                } else {
                    html.push_str("white")
                }
                html.push_str("\"'")
            }
            html.push('>');
            for cell in &row.cells {
                html.push_str("<td style='padding: 12px; color: #4a5568;");
                if self.bordered {
                    html.push_str(" border: 1px solid #e2e8f0;")
                }
                html.push_str("'>");
                html.push_str(&cell);
                html.push_str("</td>");
            }
            html.push_str("</tr>");
        }
        html.push_str("</tbody>");
        html.push_str("</table>");
        html
    }
}
