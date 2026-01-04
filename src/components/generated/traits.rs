#![allow(clippy::all)]
#![allow(noop_method_call)]
use super::vnode::VNode;

pub trait Renderable {
    fn render(&self) -> String;
}

pub trait RenderableVNode {
    fn to_vnode(&self) -> VNode;
}
