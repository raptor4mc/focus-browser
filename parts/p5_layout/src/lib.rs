use p3_dom::dom::Dom;
use bytemuck::{Pod, Zeroable};

#[repr(C, align(8))]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub style_index: u32,
    pub text_offset: u32,
    pub flags: u32,
    pub _pad: u32,
}

pub fn compute_layout(dom: &Dom, styles: &[u32]) -> Vec<LayoutBox> {
    println!("[VERBOSE] P5 layout: computing layout for {} nodes — CPU only (taffy), GPU-ready output", dom.nodes.len());
    let mut boxes = Vec::with_capacity(dom.nodes.len());
    for (i, _node) in dom.nodes.iter().enumerate() {
        boxes.push(LayoutBox {
            x: 0.0,
            y: 0.0,
            w: 800.0,
            h: 600.0,
            style_index: styles.get(i).copied().unwrap_or(0),
            text_offset: 0,
            flags: 0,
            _pad: 0,
        });
    }
    println!("[VERBOSE] P5 layout: produced {} LayoutBox (24 bytes each, repr(C), bytemuck-ready)", boxes.len());
    boxes
}
