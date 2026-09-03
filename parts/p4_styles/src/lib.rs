pub trait TNode {
    fn as_element(&self) -> Option<&p3_dom::dom::Node>;
    fn style_index(&self) -> u32;
}

impl TNode for p3_dom::dom::Node {
    fn as_element(&self) -> Option<&p3_dom::dom::Node> {
        if self.flags & 0x01 != 0 { Some(self) } else { None }
    }
    fn style_index(&self) -> u32 {
        self.style_index
    }
}
