#![allow(clippy::all)]
#![allow(noop_method_call)]
#[allow(unused_imports)]
use super::*;

use super::traits::Renderable;

/// Primary nav with baked-in active hub (F1). Source: `components_wj/shellnav.wj`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ShellNavLink {
    pub label: String,
    pub href: String,
    pub active: bool,
    pub personas: String,
}

impl ShellNavLink {
    #[inline]
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> ShellNavLink {
        ShellNavLink {
            label: label.into(),
            href: href.into(),
            active: false,
            personas: String::new(),
        }
    }

    #[inline]
    pub fn active(mut self, active: bool) -> ShellNavLink {
        self.active = active;
        self
    }

    #[inline]
    pub fn personas(mut self, personas: impl Into<String>) -> ShellNavLink {
        self.personas = personas.into();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ShellNav {
    pub links: Vec<ShellNavLink>,
}

impl ShellNav {
    #[inline]
    pub fn new() -> ShellNav {
        ShellNav { links: Vec::new() }
    }

    #[inline]
    pub fn link(mut self, link: ShellNavLink) -> ShellNav {
        self.links.push(link);
        self
    }
}

impl Renderable for ShellNav {
    #[inline]
    fn render(&self) -> String {
        let mut items = String::new();
        for link in &self.links {
            let cls = if link.active {
                " class=\"is-active\""
            } else {
                ""
            };
            let personas_attr = if link.personas.is_empty() {
                String::new()
            } else {
                format!(" data-personas=\"{}\"", link.personas)
            };
            items.push_str(&format!(
                "<a href=\"{}\"{}{}>{}</a>\n",
                link.href, personas_attr, cls, link.label
            ));
        }
        format!(
            "<nav class=\"shell-nav\" id=\"shellNav\" aria-label=\"Primary\">{}</nav>",
            items
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_nav_bakes_active_class() {
        let html = ShellNav::new()
            .link(
                ShellNavLink::new("Home", "#/")
                    .active(true)
                    .personas("owner bookkeeper auditor"),
            )
            .link(ShellNavLink::new("Money", "#/money").personas("owner bookkeeper"))
            .render();
        assert!(html.contains("id=\"shellNav\""));
        assert!(html.contains("is-active"));
        let home = html.find("href=\"#/\"").unwrap();
        let active = html.find("is-active").unwrap();
        let money = html.find("href=\"#/money\"").unwrap();
        assert!(home < active && active < money);
    }
}
