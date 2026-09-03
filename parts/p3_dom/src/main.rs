mod dom;

fn main() {
    println!("P3 DOM: greenfield flat-array DOM — aarch64 only, no Rc/RefCell/Box/Arc");
    let mut dom = dom::Dom::new();
    println!("Pre-allocated nodes: {}, children: {}, string arena: {} bytes",
        dom.nodes.capacity(), dom.children.capacity(), dom.string_arena.capacity());
    println!("No DOM API to JS. No CSSOM. No Shadow DOM. No WASM. Subtitles external.");
}
