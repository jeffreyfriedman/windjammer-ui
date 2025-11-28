#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::traits::Renderable;

pub struct Pagination {
    current_page: i32,
    total_pages: i32,
    show_first_last: bool,
    show_prev_next: bool,
}

impl Pagination {
    #[inline]
    pub fn new(current_page: i32, total_pages: i32) -> Pagination {
        Pagination {
            current_page,
            total_pages,
            show_first_last: true,
            show_prev_next: true,
        }
    }
    #[inline]
    pub fn show_first_last(mut self, show: bool) -> Pagination {
        self.show_first_last = show;
        self
    }
    #[inline]
    pub fn show_prev_next(mut self, show: bool) -> Pagination {
        self.show_prev_next = show;
        self
    }
}

impl Renderable for Pagination {
    fn render(self) -> String {
        let mut html = "<nav class='wj-pagination'><ul>".to_string();
        if self.show_first_last {
            html = format!(
                "{}<li class='wj-pagination-item'><a href='#'>«</a></li>",
                html
            );
        }
        if self.show_prev_next {
            let prev_disabled = {
                if self.current_page == 1 {
                    " disabled"
                } else {
                    ""
                }
            };
            html = format!(
                "{}<li class='wj-pagination-item{}'><a href='#'>‹</a></li>",
                html, prev_disabled
            );
        }
        let mut page = 1;
        while page <= self.total_pages {
            let active = {
                if page == self.current_page {
                    " active"
                } else {
                    ""
                }
            };
            html = format!(
                "{}<li class='wj-pagination-item{}'><a href='#'>{}</a></li>",
                html, active, page
            );
            page += 1;
        }
        if self.show_prev_next {
            let next_disabled = {
                if self.current_page == self.total_pages {
                    " disabled"
                } else {
                    ""
                }
            };
            html = format!(
                "{}<li class='wj-pagination-item{}'><a href='#'>›</a></li>",
                html, next_disabled
            );
        }
        if self.show_first_last {
            html = format!(
                "{}<li class='wj-pagination-item'><a href='#'>»</a></li>",
                html
            );
        }
        format!("{}</ul></nav>", html)
    }
}
