#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Pagination {
    pub current_page: i32,
    pub total_pages: i32,
    pub show_first_last: bool,
    pub show_prev_next: bool,
}
impl Pagination {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut __bytes = Vec::with_capacity(16);
        __bytes.extend_from_slice(&self.current_page.to_ne_bytes());
        __bytes.extend_from_slice(&self.total_pages.to_ne_bytes());
        __bytes.extend_from_slice(&(if self.show_first_last { 1u32 } else { 0u32 }).to_ne_bytes());
        __bytes.extend_from_slice(&(if self.show_prev_next { 1u32 } else { 0u32 }).to_ne_bytes());
        __bytes
    }
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
    #[inline]
    fn render(&self) -> String {
        let mut html = "<nav class='wj-pagination'><ul>".to_string();
        if self.show_first_last {
            html = format!(
                "{}<li class='wj-pagination-item'><a href='#'>«</a></li>",
                html
            );
        }
        if self.show_prev_next {
            let prev_disabled = {
                if self.current_page == 1_i32 {
                    String::from(" disabled")
                } else {
                    String::new()
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
                    String::from(" active")
                } else {
                    String::new()
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
                    String::from(" disabled")
                } else {
                    String::new()
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
