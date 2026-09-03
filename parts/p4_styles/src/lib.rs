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

pub fn compute_styles(dom: &p3_dom::dom::Dom, _stylesheet: &str) -> Vec<u32> {
    println!("P4 styles: computing styles for {} nodes — no CSSOM, compute once", dom.nodes.len());
    let mut results = Vec::with_capacity(dom.nodes.len());
    for (i, _node) in dom.nodes.iter().enumerate() {
        results.push(i as u32);
    }
    results
}
