use crate::geom::Rect;
use crate::tree::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Role {
    /// No semantic presence; children are hoisted into the nearest semantic ancestor.
    #[default]
    None,
    Group,
    Text,
    Heading,
    Button,
    Checkbox,
    TextField,
    List,
    ListItem,
    ScrollView,
    Image,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Click,
    Focus,
    SetValue,
    ScrollIntoView,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Semantics {
    pub role: Role,
    pub name: Option<String>,
    pub value: Option<String>,
    pub checked: Option<bool>,
    pub disabled: bool,
    pub focusable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticNode {
    pub id: NodeId,
    pub role: Role,
    pub name: Option<String>,
    pub value: Option<String>,
    pub checked: Option<bool>,
    pub disabled: bool,
    pub focusable: bool,
    pub actions: Vec<Action>,
    /// Absolute bounds in scene coordinates.
    pub bounds: Rect,
    pub children: Vec<SemanticNode>,
}

/// The accessibility-facing view of a scene, produced on demand from the node tree.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticTree {
    pub roots: Vec<SemanticNode>,
}

impl SemanticTree {
    pub fn count(&self) -> usize {
        fn n(s: &SemanticNode) -> usize {
            1 + s.children.iter().map(n).sum::<usize>()
        }
        self.roots.iter().map(n).sum()
    }
    /// Depth-first iteration.
    pub fn walk<'a>(&'a self, f: &mut dyn FnMut(&'a SemanticNode)) {
        fn go<'a>(s: &'a SemanticNode, f: &mut dyn FnMut(&'a SemanticNode)) {
            f(s);
            for c in &s.children {
                go(c, f);
            }
        }
        for r in &self.roots {
            go(r, f);
        }
    }
}
