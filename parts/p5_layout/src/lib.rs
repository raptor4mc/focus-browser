use p3_dom::dom::Dom;
use bytemuck::{Pod, Zeroable};
use taffy::prelude::*;

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

    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let root_style = Style {
        size: Size {
            width: length(800.0),
            height: length(600.0),
        },
        ..Default::default()
    };

    let root = taffy.new_leaf(root_style).expect("taffy root");

    taffy.compute_layout(
        root,
        Size {
            width: AvailableSpace::Definite(800.0),
            height: AvailableSpace::Definite(600.0),
        },
    ).expect("taffy compute");

    let layout = taffy.layout(root).expect("taffy layout");

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
