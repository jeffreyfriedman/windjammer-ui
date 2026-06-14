#[allow(unused_imports)]
use super::*;
use windjammer_runtime::test::*;

use super::node_graph::NodeGraph;
#[test]
#[inline]
pub fn test_apply_execution_trace_highlights_nodes() {
    let mut ids: Vec<String> = Vec::new();
    ids.push("spawn_wave".to_string());
    ids.push("wait_delay".to_string());
    let graph = NodeGraph::new().apply_execution_trace(ids.clone());
    assert_eq!(graph.trace_node_ids.len(), 2);
}

#[test]
#[inline]
pub fn test_trace_highlight_sets_active_class() {
    let mut node = super::node_graph::GraphNode::new("n1".to_string(), "Spawn".to_string(), super::node_graph::NodeCategory::Event);
    node = node.trace_highlight(true);
    assert!(node.trace_active, "trace highlight flag set");
}

