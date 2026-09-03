mod dom;
mod parser;

fn main() {
    println!("P3 DOM: greenfield flat-array DOM — aarch64 only, no Rc/RefCell/Box/Arc");
    let mut dom = dom::Dom::new();
    println!("Pre-allocated nodes: {}, children: {}, string arena: {} bytes",
        dom.nodes.capacity(), dom.children.capacity(), dom.string_arena.capacity());

    let html = r#"<!DOCTYPE html><html><head><title>Test</title></head><body><div id="main"><p>Hello <b>world</b></p></div></body></html>"#;
    println!("Parsing HTML: {}", html);

    parser::parse_html(html, &mut dom);

    println!("Node count: {}", dom.nodes.len());
    for (i, node) in dom.nodes.iter().enumerate() {
        let tag_name = if node.flags & 0x01 != 0 {
            format!("tag_{}", node.tag)
        } else {
            "text".to_string()
        };
        println!("Node {}: {} | parent={} | flags={:#06x}", i, tag_name, dom.parent[i], node.flags);
    }

    println!("No DOM API to JS. No CSSOM. No Shadow DOM. No WASM. Subtitles external.");
}
