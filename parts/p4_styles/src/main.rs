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
    println!("P4 styles: stylo TNode trait for flat-array DOM — no CSSOM, no JS access");
}
