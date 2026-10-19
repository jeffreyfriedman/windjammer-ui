use std::collections::HashMap;

use accesskit::{Action, Node, NodeId, Rect, Role, Toggled, TreeId, TreeInfo, TreeUpdate};
use wj_scene::{Action as SAction, NodeId as SId, Role as SRole, SemanticNode, SemanticTree};

/// The synthetic window node that parents the scene's semantic roots.
pub const ROOT_ID: NodeId = NodeId(0);

fn ak_id(id: SId) -> NodeId {
    // Scene ids are (index, generation); AccessKit ids are u64. Index+1 keeps 0 for the window.
    let (idx, gen) = id.parts();
    NodeId(((gen as u64) << 32) | (idx as u64 + 1))
}

fn role(r: SRole) -> Role {
    match r {
        SRole::None | SRole::Group => Role::GenericContainer,
        SRole::Text => Role::Label,
        SRole::Heading => Role::Heading,
        SRole::Button => Role::Button,
        SRole::Checkbox => Role::CheckBox,
        SRole::TextField => Role::TextInput,
        SRole::List => Role::List,
        SRole::ListItem => Role::ListItem,
        SRole::ScrollView => Role::ScrollView,
        SRole::Image => Role::Image,
    }
}

fn convert(n: &SemanticNode) -> Node {
    let mut node = Node::new(role(n.role));
    if let Some(name) = &n.name {
        node.set_label(name.clone());
    }
    if let Some(v) = &n.value {
        node.set_value(v.clone());
    }
    if let Some(c) = n.checked {
        node.set_toggled(if c { Toggled::True } else { Toggled::False });
    }
    if n.disabled {
        node.set_disabled();
    }
    node.set_bounds(Rect { x0: n.bounds.x as f64, y0: n.bounds.y as f64, x1: (n.bounds.x + n.bounds.w) as f64, y1: (n.bounds.y + n.bounds.h) as f64 });
    for a in &n.actions {
        node.add_action(match a {
            SAction::Click => Action::Click,
            SAction::Focus => Action::Focus,
            SAction::SetValue => Action::SetValue,
            SAction::ScrollIntoView => Action::ScrollIntoView,
        });
    }
    node.set_children(n.children.iter().map(|c| ak_id(c.id)).collect::<Vec<_>>());
    node
}

fn flatten(n: &SemanticNode, out: &mut Vec<(NodeId, Node)>) {
    out.push((ak_id(n.id), convert(n)));
    for c in &n.children {
        flatten(c, out);
    }
}

/// Keeps the last tree sent so each frame produces the smallest valid update.
#[derive(Default)]
pub struct AccessKitTree {
    sent: HashMap<NodeId, Node>,
    last_focus: Option<NodeId>,
    started: bool,
}

impl AccessKitTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the update for `tree`; `None` when nothing changed since the last call.
    pub fn update(&mut self, tree: &SemanticTree, focus: Option<SId>) -> Option<TreeUpdate> {
        let mut flat = vec![];
        for r in &tree.roots {
            flatten(r, &mut flat);
        }
        let mut root = Node::new(Role::Window);
        root.set_children(tree.roots.iter().map(|r| ak_id(r.id)).collect::<Vec<_>>());
        flat.push((ROOT_ID, root));

        let focus_id = focus.map(ak_id).filter(|f| flat.iter().any(|(id, _)| id == f)).unwrap_or(ROOT_ID);
        let changed: Vec<(NodeId, Node)> = flat.iter().filter(|(id, n)| self.sent.get(id) != Some(n)).cloned().collect();
        let removed = self.sent.keys().any(|id| !flat.iter().any(|(i, _)| i == id));
        if self.started && changed.is_empty() && !removed && self.last_focus == Some(focus_id) {
            return None;
        }
        let first = !self.started;
        self.sent = flat.into_iter().collect();
        self.last_focus = Some(focus_id);
        self.started = true;
        let mut info = TreeInfo::new(ROOT_ID);
        info.toolkit_name = Some("windjammer-ui".into());
        Some(TreeUpdate { nodes: changed, tree: if first { Some(info) } else { None }, tree_id: TreeId::ROOT, focus: focus_id })
    }

    /// Nodes currently known to the platform adapter (for leak checks).
    pub fn known(&self) -> usize {
        self.sent.len()
    }
}
