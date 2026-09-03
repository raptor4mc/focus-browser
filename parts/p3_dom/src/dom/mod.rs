#[repr(C, align(64))]
pub struct Node {
    pub tag: u16,
    pub attrs: u32,
    pub text: u32,
    pub flags: u16,
    pub style_index: u32,
    pub layout_index: u32,
    pub _pad: u32,
}

pub struct Dom {
    pub nodes: Vec<Node>,
    pub children: Vec<u32>,
    pub children_start: Vec<u32>,
    pub parent: Vec<u32>,
    pub string_arena: Vec<u8>,
    pub attr_arena: Vec<u8>,
    pub interned_tags: [u16; 256],
}

impl Dom {
    pub fn new() -> Self {
        Self {
            nodes: Vec::with_capacity(10_000),
            children: Vec::with_capacity(50_000),
            children_start: Vec::with_capacity(10_000),
            parent: Vec::with_capacity(10_000),
            string_arena: Vec::with_capacity(1 << 20),
            attr_arena: Vec::with_capacity(1 << 20),
            interned_tags: [0; 256],
        }
    }

    pub fn push_node(&mut self, tag: u16, flags: u16) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(Node {
            tag,
            attrs: 0,
            text: 0,
            flags,
            style_index: 0,
            layout_index: 0,
            _pad: 0,
        });
        self.children_start.push(0);
        self.parent.push(0);
        idx
    }
}
