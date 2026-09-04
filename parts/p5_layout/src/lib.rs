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
    println!("[VERBOSE] P5 layout: computing layout with taffy for {} nodes", dom.nodes.len());
    let mut tree = taffy::Tree::new();
    let root_style = taffy::Style {
        size: taffy::Size {
            width: taffy::Dimension::Length(800.0),
            height: taffy::Dimension::Length(600.0),
        },
        ..Default::default()
    };
    let root = tree.new_leaf(root_style).expect("taffy root");
    tree.compute_layout(
        root,
        taffy::geometry::Size {
            width: Some(800.0),
            height: Some(600.0),
        },
    ).expect("taffy compute");
    let layout = tree.layout(root).expect("taffy layout");

    let mut boxes = Vec::with_capacity(dom.nodes.len());
    for (i, _node) in dom.nodes.iter().enumerate() {
        boxes.push(LayoutBox {
            x: layout.location.x,
            y: layout.location.y,
            w: layout.size.width,
            h: layout.size.height,
            style_index: styles.get(i).copied().unwrap_or(0),
            text_offset: 0,
            flags: 0,
            _pad: 0,
        });
    }
    println!("[VERBOSE] P5 layout: produced {} LayoutBox via taffy (repr(C), bytemuck-ready)", boxes.len());
    boxes
}
