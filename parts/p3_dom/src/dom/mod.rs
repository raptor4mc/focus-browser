pub mod parser;

use markup5ever::LocalName;
use std::collections::HashMap;

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

pub struct NodeFlags;
impl NodeFlags {
    pub const IS_ELEMENT: u16 = 0x01;
    pub const IS_TEXT: u16 = 0x02;
}

pub struct Dom {
    pub nodes: Vec<Node>,
    pub children: Vec<u32>,
    pub children_start: Vec<u32>,
    pub parent: Vec<u32>,
    pub string_arena: Vec<u8>,
    pub attr_arena: Vec<u8>,
    pub tag_names: Vec<LocalName>,
    pub tag_map: HashMap<String, u16>,
    pub next_tag_id: u16,
}

impl Dom {
    pub fn new() -> Self {
        const INITIAL_NODES: usize = 10_000;
        const INITIAL_CHILDREN: usize = 50_000;
        const INITIAL_STRING_ARENA: usize = 1 << 20;
        Self {
            nodes: Vec::with_capacity(INITIAL_NODES),
            children: Vec::with_capacity(INITIAL_CHILDREN),
            children_start: Vec::with_capacity(INITIAL_NODES),
            parent: Vec::with_capacity(INITIAL_NODES),
            string_arena: Vec::with_capacity(INITIAL_STRING_ARENA),
            attr_arena: Vec::with_capacity(INITIAL_STRING_ARENA),
            tag_names: Vec::new(),
            tag_map: HashMap::new(),
            next_tag_id: 1,
        }
    }

    pub fn intern_tag(&mut self, name: &str) -> u16 {
        if let Some(&id) = self.tag_map.get(name) {
            return id;
        }
        let id = self.next_tag_id;
        self.tag_names.push(LocalName::from(name));
        self.tag_map.insert(name.to_string(), id);
        self.next_tag_id += 1;
        id
    }

    pub fn store_attrs(&mut self, attrs: &Vec<html5ever::Attribute>) -> u32 {
        let offset = self.attr_arena.len() as u32;
        self.attr_arena.extend_from_slice(&(attrs.len() as u32).to_le_bytes());
        offset
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
        self.parent.push(u32::MAX);
        idx
    }
}
