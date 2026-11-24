use super::a11y::*;
use super::vnode::VNode;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct GridColumn {
    pub header: String,
    pub numeric: bool,
}

impl GridColumn {
#[inline]
pub fn text(header: String) -> GridColumn {
        GridColumn { header, numeric: false }
}
#[inline]
pub fn number(header: String) -> GridColumn {
        GridColumn { header, numeric: true }
}
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct DataGrid {
    pub base: String,
    pub caption: String,
    pub columns: Vec<GridColumn>,
    pub rows: Vec<Vec<String>>,
    pub sort_col: i32,
    pub ascending: bool,
    pub filter: String,
    pub page: i32,
    pub page_size: i32,
    pub focus: String,
}

impl DataGrid {
#[inline]
pub fn new(base: String, caption: String) -> DataGrid {
        DataGrid { base, caption, columns: Vec::new(), rows: Vec::new(), sort_col: -1_i32, ascending: true, filter: String::new(), page: 1_i32, page_size: 10_i32, focus: String::new() }
}
#[inline]
pub fn column(mut self, c: GridColumn) -> DataGrid {
        self.columns.push(c);
        self
}
#[inline]
pub fn row(mut self, cells: Vec<String>) -> DataGrid {
        self.rows.push(cells);
        self
}
#[inline]
pub fn page_size(mut self, n: i32) -> DataGrid {
        self.page_size = n;
        self
}
#[inline]
pub fn current_page(&self) -> i32 {
        self.page
}
/// Indices into the row list that pass the filter, in sort order (before paging).

pub fn matching_rows(&self) -> Vec<i32> {
        let needle = self.filter.to_lowercase();
        let mut order: Vec<i32> = Vec::new();
        let mut i = 0_usize;
        while i < self.rows.len() {
            let mut hit = needle.is_empty();
            let mut c = 0_usize;
            while c < self.rows[i].len() && !hit {
                if self.rows[i][c].to_lowercase().contains(&needle) {
                    hit = true;
                }
                c += 1;
            }
            if hit {
                order.push(i as i32);
            }
            i = i.wrapping_add(1);
        }
        if self.sort_col >= 0_i32 {
            let col = self.sort_col as usize;
            let numeric = self.columns[col].numeric;
            let mut a = 1_usize;
            while a < order.len() {
                let mut b = a;
                while b > 0_usize {
                    let left = (&self.rows[order[b - 1] as usize][col]).to_string();
                    let right = (&self.rows[order[b] as usize][col]).to_string();
                    let mut c = compare_cells(&left, &right, numeric);
                    if !self.ascending {
                        c = 0_i32 - c;
                    }
                    if c <= 0_i32 {
                        break;
                    }
                    let tmp = order[b - 1];
                    order[b - 1] = order[b];
                    order[b] = tmp;
                    b -= 1;
                }
                a = a.wrapping_add(1);
            }
        }
        order
}
#[inline]
pub fn page_count(&self) -> i32 {
        let n = self.matching_rows().len() as i32;
        if n == 0_i32 {
            return 1_i32;
        }
        ((n + self.page_size - 1_i32) / self.page_size) as i32
}
#[inline]
pub fn sort_by(&mut self, col: i32) {
        if self.sort_col == col {
            if self.ascending {
                self.ascending = false;
            } else {
                self.sort_col = -1_i32;
                self.ascending = true;
            }
        } else {
            self.sort_col = col;
            self.ascending = true;
        }
        self.page = 1_i32;
        self.focus = part_id(&self.base, "sort", col);
}
/// Rows on the current page, as indices into the row list.
#[inline]
pub fn visible_rows(&self) -> Vec<i32> {
        let all = self.matching_rows();
        let start = (self.page - 1_i32) * self.page_size;
        let mut out: Vec<i32> = Vec::new();
        let mut k = start;
        while k < start + self.page_size && k < (all.len() as i32) {
            out.push(all[k as usize]);
            k += 1;
        }
        out
}
#[inline]
pub fn go(&mut self, page: i32) {
        let last = self.page_count();
        self.page = {
            if page < 1_i32 {
                1_i32
            } else {
                if page > last {
                    last
                } else {
                    page
                }
            }
        };
        self.focus = String::new();
}
}

impl Interactive for DataGrid {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act == "filter" && event == "input" {
            self.filter = value;
            self.page = 1_i32;
            return false;
        }
        let col = act_index(&act, "sort");
        if col >= 0_i32 {
            if event == "click" || event == "keydown" && is_activation_key(&key) {
                let _ = self.sort_by(col);
                return true;
            }
            return false;
        }
        let activate = event == "click" || event == "keydown" && is_activation_key(&key);
        if act == "prev" && activate {
            let _ = self.go(self.page - 1_i32);
            return true;
        }
        if act == "next" && activate {
            let _ = self.go(self.page + 1_i32);
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        self.focus.clone()
}

fn view(&self) -> VNode {
        let rows = self.visible_rows();
        let total = self.matching_rows().len() as i32;
        let first = {
            if total == 0_i32 {
                0_i32
            } else {
                (self.page - 1_i32) * self.page_size + 1_i32
            }
        };
        let last = first + rows.len() as i32 - 1_i32;
        let filter = VNode::input().add_class("wj-input".to_string()).set_id(part_id(&self.base, "filter", 0_i32)).set_type("search".to_string()).set_value(self.filter.clone()).add_attr("aria-label".to_string(), "Filter rows".to_string()).add_attr("aria-controls".to_string(), part_id(&self.base, "table", 0_i32)).add_attr("data-wj-act".to_string(), "filter".to_string());
        let mut head = VNode::element("tr".to_string());
        let mut c = 0_i32;
        while c < (self.columns.len() as i32) {
            let sorted = c == self.sort_col;
            let dir: String = {
                if !sorted {
                    String::from("none")
                } else {
                    if self.ascending {
                        String::from("ascending")
                    } else {
                        String::from("descending")
                    }
                }
            };
            head = { head.child({ VNode::element("th".to_string()).add_attr("scope".to_string(), "col".to_string()).add_attr("aria-sort".to_string(), dir.to_string()).child({ let _temp0 = format!("sort:{}", c); VNode::button().add_class("wj-sort".to_string()).set_id(part_id(&self.base, "sort", c)).add_attr("type".to_string(), "button".to_string()).add_attr("data-wj-act".to_string(), _temp0) }.add_text(self.columns[c as usize].header.clone())) }) };
            c = c.wrapping_add(1);
        }
        let mut body = VNode::element("tbody".to_string());
        let mut r = 0_usize;
        while r < rows.len() {
            let mut tr = VNode::element("tr".to_string());
            let cells = &self.rows[rows[r] as usize];
            let mut k = 0_usize;
            while k < cells.len() {
                tr = tr.child(VNode::element("td".to_string()).add_text(cells[k].clone()));
                k = k.wrapping_add(1);
            }
            body = body.child(tr);
            r = r.wrapping_add(1);
        }
        let table = { let _temp0 = format!("{}", self.columns.len()); { let _temp0 = format!("{}", total); VNode::element("table".to_string()).add_class("wj-table".to_string()).set_id(part_id(&self.base, "table", 0_i32)).add_attr("aria-rowcount".to_string(), _temp0) }.add_attr("aria-colcount".to_string(), _temp0) }.child(VNode::element("caption".to_string()).add_text(self.caption.clone())).child(VNode::element("thead".to_string()).child(head)).child(body);
        let status = { VNode::div().add_class("wj-grid-status".to_string()).set_id(part_id(&self.base, "status", 0_i32)).add_attr("role".to_string(), "status".to_string()).add_attr("aria-live".to_string(), "polite".to_string()).add_text({
            if total == 0_i32 {
                String::from("No rows match")
            } else {
                format!("Showing {}-{} of {}", first, last, total)
            }
        }) };
        let mut prev = VNode::button().add_class("wj-page".to_string()).set_id(part_id(&self.base, "prev", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("data-wj-act".to_string(), "prev".to_string()).add_text("Prev".to_string());
        if self.page <= 1_i32 {
            prev = prev.add_attr("disabled".to_string(), "true".to_string());
        }
        let mut next = VNode::button().add_class("wj-page".to_string()).set_id(part_id(&self.base, "next", 0_i32)).add_attr("type".to_string(), "button".to_string()).add_attr("data-wj-act".to_string(), "next".to_string()).add_text("Next".to_string());
        if self.page >= self.page_count() {
            next = next.add_attr("disabled".to_string(), "true".to_string());
        }
        let pager = { VNode::element("nav".to_string()).add_class("wj-pagination".to_string()).add_attr("aria-label".to_string(), "Table pages".to_string()).child(prev).child({ let _temp0 = format!("Page {} of {}", self.page, self.page_count()); VNode::span().add_text(_temp0) }) }.child(next);
        VNode::div().add_class("wj-datagrid".to_string()).child(filter).child(table).child(status).child(pager)
}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct VirtualList {
    pub base: String,
    pub label: String,
    pub count: i32,
    pub row_height: i32,
    pub viewport: i32,
    pub overscan: i32,
    pub scroll_top: i32,
    pub active: i32,
}

impl VirtualList {
/// A list of `count` rows, each `row_height` px tall, shown through a `viewport` px tall window.
#[inline]
pub fn new(base: String, label: String, count: i32, row_height: i32, viewport: i32) -> VirtualList {
        VirtualList { base, label, count, row_height, viewport, overscan: 3_i32, scroll_top: 0_i32, active: 0_i32 }
}
#[inline]
pub fn active_index(&self) -> i32 {
        self.active
}
#[inline]
pub fn first_rendered(&self) -> i32 {
        let first = self.scroll_top / self.row_height - self.overscan;
        if first < 0_i32 {
            0_i32
        } else {
            first
        }
}
#[inline]
pub fn last_rendered(&self) -> i32 {
        let last = (self.scroll_top + self.viewport) / self.row_height + self.overscan;
        if last >= self.count {
            self.count - 1_i32
        } else {
            last
        }
}
#[inline]
pub fn rows_per_page(&self) -> i32 {
        let n = self.viewport / self.row_height;
        if n < 1_i32 {
            1_i32
        } else {
            n
        }
}
#[inline]
pub fn move_to(&mut self, index: i32) {
        let mut next = index;
        if next < 0_i32 {
            next = 0_i32;
        }
        if next > self.count - 1_i32 {
            next = self.count - 1_i32;
        }
        self.active = next;
        let top = next * self.row_height;
        if top < self.scroll_top {
            self.scroll_top = top;
        }
        if top + self.row_height > self.scroll_top + self.viewport {
            self.scroll_top = top + self.row_height - self.viewport;
        }
}
}

impl Interactive for VirtualList {

fn on_event(&mut self, event: String, act: String, key: String, value: String) -> bool {
        if act != "vlist" {
            return false;
        }
        if event == "scroll" {
            self.scroll_top = value.parse::<i32>().unwrap_or(self.scroll_top);
            return false;
        }
        if event != "keydown" {
            return false;
        }
        if key == "ArrowDown" {
            let _ = self.move_to(self.active + 1_i32);
            return true;
        }
        if key == "ArrowUp" {
            let _ = self.move_to(self.active - 1_i32);
            return true;
        }
        if key == "PageDown" {
            let _ = self.move_to(self.active + self.rows_per_page());
            return true;
        }
        if key == "PageUp" {
            let _ = self.move_to(self.active - self.rows_per_page());
            return true;
        }
        if key == "Home" {
            let _ = self.move_to(0_i32);
            return true;
        }
        if key == "End" {
            let _ = self.move_to(self.count - 1_i32);
            return true;
        }
        false
}
#[inline]
fn focus_id(&self) -> String {
        String::new()
}

fn view(&self) -> VNode {
        let mut spacer = { let _temp0 = format!("position: relative; height: {}px", self.count * self.row_height); VNode::div().add_style(_temp0) };
        let mut i = self.first_rendered();
        let last = self.last_rendered();
        while i <= last {
            let row = { let _temp0 = format!("Row {}", i + 1_i32); { let _temp0 = format!("position: absolute; left: 0; right: 0; top: {}px; height: {}px", i * self.row_height, self.row_height); { let _temp0 = format!("{}", self.count); { let _temp0 = format!("{}", i + 1_i32); VNode::div().add_class("wj-vlist-row".to_string()).set_id(part_id(&self.base, "row", i)).add_attr("role".to_string(), "option".to_string()).add_attr("aria-posinset".to_string(), _temp0) }.add_attr("aria-setsize".to_string(), _temp0) }.add_attr("aria-selected".to_string(), aria_bool(i == self.active)).add_style(_temp0) }.add_text(_temp0) };
            spacer = spacer.child(row);
            i += 1;
        }
        { let _temp0 = format!("overflow: auto; height: {}px", self.viewport); { let _temp0 = format!("{}", self.scroll_top); VNode::div().add_class("wj-vlist".to_string()).set_id(part_id(&self.base, "list", 0_i32)).add_attr("role".to_string(), "listbox".to_string()).add_attr("tabindex".to_string(), "0".to_string()).add_attr("aria-label".to_string(), self.label.clone()).add_attr("aria-activedescendant".to_string(), part_id(&self.base, "row", self.active)).add_attr("data-wj-act".to_string(), "vlist".to_string()).add_attr("data-wj-scroll-top".to_string(), _temp0) }.add_style(_temp0) }.child(spacer)
}
}

/// Orders two cells: numerically when both parse as numbers and `numeric` is set, otherwise as lowercase text.
#[inline]
pub fn compare_cells(a: &str, b: &str, numeric: bool) -> i32 {
    if numeric {
        let x = a.trim().parse::<f64>().unwrap_or(0.0_f64);
        let y = b.trim().parse::<f64>().unwrap_or(0.0_f64);
        if x < y {
            return -1_i32;
        }
        if x > y {
            return 1_i32;
        }
        return 0_i32;
    }
    let x = a.to_lowercase();
    let y = b.to_lowercase();
    if x < y {
        return -1_i32;
    }
    if x > y {
        return 1_i32;
    }
    0_i32
}

