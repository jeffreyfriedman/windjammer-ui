#[allow(unused_imports)]
use super::*;

use super::vnode::VNode;
pub trait Renderable {
    fn render(&self) -> String;
}

pub trait RenderableVNode {
    fn to_vnode(&self) -> VNode;
}

