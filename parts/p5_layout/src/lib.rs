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
    let mut taffy_ids = Vec::with_capacity(dom.nodes.len());

    // Phase 1: create a taffy leaf for every DOM node
    for _ in 0..dom.nodes.len() {
        let style = Style {
            display: Display::Block,
            size: Size::auto(),
            ..Default::default()
        };
        let id = taffy.new_leaf(style).expect("taffy new_leaf");
        taffy_ids.push(id);
    }

    // Phase 2: wire parent → child edges using dom.parent array
    for i in 0..dom.nodes.len() {
        let parent_idx = dom.parent[i];
        if parent_idx != u32::MAX && (parent_idx as usize) < dom.nodes.len() {
            let parent_id = taffy_ids[parent_idx as usize];
            let child_id = taffy_ids[i];
            let _ = taffy.add_child(parent_id, child_id);
        }
    }

    // Phase 3: compute layout from root
    let root_id = taffy_ids[0];
    taffy.compute_layout(
        root_id,
        Size {
            width: AvailableSpace::Definite(800.0),
            height: AvailableSpace::Definite(600.0),
        },
    ).expect("taffy compute");

    // Phase 4: collect per-node layouts
    let mut boxes = Vec::with_capacity(dom.nodes.len());
    for i in 0..dom.nodes.len() {
        let layout = taffy.layout(taffy_ids[i]).expect("taffy layout");
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
