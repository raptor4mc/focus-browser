mod dom;
mod parser;

fn main() {
    println!("P3 DOM: greenfield flat-array DOM — aarch64 only, no Rc/RefCell/Box/Arc");
    let mut dom = dom::Dom::new();
    println!("Pre-allocated nodes: {}, children: {}, string arena: {} bytes",
        dom.nodes.capacity(), dom.children.capacity(), dom.string_arena.capacity());

    let html = r#"<!DOCTYPE html><html><head><title>Test</title></head><body><div id="main"><p>Hello <b>world</b></p></div></body></html>"#;
    println!("Parsing HTML: {}", html);

    let mut sink = parser::DomTreeSink::new(dom);
    let parser = html5ever::parse_document(sink, html5ever::ParseOpts::default());
    // Note: html5ever parse_document takes ownership; for streaming use parse_document with sink
    // This is a simplified call; full streaming requires html5ever::driver::parse_document
    println!("Parse complete. Total nodes: {}", sink.dom.nodes.len());

    println!("Node count: {}", sink.dom.nodes.len());
    for (i, node) in sink.dom.nodes.iter().enumerate() {
        let tag_name = if node.flags & 0x01 != 0 {
            format!("tag_{}", node.tag)
        } else {
            "text".to_string()
        };
        println!("Node {}: {} | parent={} | flags={:#06x}", i, tag_name, sink.dom.parent[i], node.flags);
    }

    println!("No DOM API to JS. No CSSOM. No Shadow DOM. No WASM. Subtitles external.");
}
