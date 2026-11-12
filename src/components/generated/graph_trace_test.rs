#[allow(unused_imports)]
use super::*;
use windjammer_runtime::test::*;

use super::node_graph::NodeGraph;
#[test]

pub fn test_apply_execution_trace_highlights_nodes() {
    let mut ids: Vec<String> = Vec::new();
    ids.push(String::from("spawn_wave"));
    ids.push(String::from("wait_delay"));
    let graph = NodeGraph::new().apply_execution_trace(ids);
    assert_eq!(graph.trace_node_ids.len(), 2_usize);
}

#[test]

pub fn test_trace_highlight_sets_active_class() {
    let mut node = super::node_graph::GraphNode::new("n1".to_string(), "Spawn".to_string(), super::node_graph::NodeCategory::Event);
    node = node.trace_highlight(true);
    assert!(node.trace_active, "trace highlight flag set");
}

