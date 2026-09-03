use p3_dom::dom::{Dom, Node};

pub trait TNode {
    fn as_element(&self) -> Option<&Node>;
    fn style_index(&self) -> u32;
}

impl TNode for Node {
    fn as_element(&self) -> Option<&Node> {
        if self.flags & 0x01 != 0 { Some(self) } else { None }
    }
    fn style_index(&self) -> u32 {
        self.style_index
    }
}

fn main() {
    println!("[VERBOSE] P4 styles: stylo TNode trait — no CSSOM, no getComputedStyle, compute once");
    let mut dom = Dom::new();
    let root = dom.push_node(1, 0x01);
    println!("[VERBOSE] P4 styles: test DOM with root node {} (tag=1, IS_ELEMENT)", root);
    println!("[VERBOSE] P4 styles: computing styles for {} nodes (CPU — rayon/stylo)", dom.nodes.len());
    println!("[VERBOSE] P4 styles: no Shadow DOM. No iframes. No quirks mode. Standards only.");
}
