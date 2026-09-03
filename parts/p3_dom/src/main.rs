use html5ever::tendril::TendrilSink;

use p3_dom::dom::{Dom, parser::DomParser};

fn main() {
    let html = r#"<!DOCTYPE html><html><head><title>Test</title></head><body><div id="main"><p>Hello <b>world</b></p></div></body></html>"#;

    let dom = Dom::new();
    let parser = parse_document(DomParser::new(dom), Default::default());
    let result = parser.one(html);

    println!("Total nodes: {}", result.dom.nodes.len());
    for (i, node) in result.dom.nodes.iter().enumerate() {
        let tag = if node.flags.contains(super::NodeFlags::IS_ELEMENT) {
            result.dom.tag_names.get(node.tag as usize).map(|l| l.as_ref()).unwrap_or("unknown")
        } else {
            "text"
        };
        let parent = result.dom.parent[i];
        let child_count = if i + 1 < result.dom.children_start.len() {
            (result.dom.children_start[i + 1] - result.dom.children_start[i]) as usize
        } else { 0 };
        println!("Node {}: tag={}, parent={}, children={}", i, tag, parent, child_count);
    }
}
