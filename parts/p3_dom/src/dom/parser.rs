use html5ever::tree_builder::{TreeSink, QuirksMode, ElementFlags};
use html5ever::{Attribute, ExpandedName};
use html5ever::tendril::StrTendril;
use std::collections::HashMap;
use crate::dom::Dom;

pub struct DomTreeSink {
    pub dom: Dom,
    tag_map: HashMap<String, u16>,
    next_tag_id: u16,
}

impl DomTreeSink {
    pub fn new(dom: Dom) -> Self {
        Self {
            dom,
            tag_map: HashMap::new(),
            next_tag_id: 1,
        }
    }

    fn intern_tag(&mut self, name: &str) -> u16 {
        if let Some(&id) = self.tag_map.get(name) {
            return id;
        }
        let id = self.next_tag_id;
        self.next_tag_id += 1;
        self.tag_map.insert(name.to_string(), id);
        id
    }

    fn tag_name(&self, id: u16) -> String {
        for (k, v) &self.tag_map {
            if *v == id {
                return k.clone();
            }
        }
        "unknown".to_string()
    }
}

impl TreeSink for DomTreeSink {
    type Handle = u32;
    type ElemName<'a> = ExpandedName<'a>;
    type Attribute = Attribute;

    fn create_element(&mut self, name: Self::ElemName<'_>, attrs: Vec<Self::Attribute>, _flags: ElementFlags) -> Self::Handle {
        let tag_str = name.local.as_ref();
        let tag_id = self.intern_tag(tag_str);
        let idx = self.dom.push_node(tag_id, 0x01);
        if !attrs.is_empty() {
            let offset = self.dom.attr_arena.len() as u32;
            self.dom.attr_arena.extend_from_slice(&(attrs.len() as u32).to_le_bytes());
            self.dom.nodes[idx as usize].attrs = offset;
        }
        idx
    }

    fn create_text(&mut self, text: StrTendril, parent: Self::Handle) -> Self::Handle {
        let offset = self.dom.string_arena.len() as u32;
        self.dom.string_arena.extend_from_slice(text.as_bytes());
        let idx = self.dom.push_node(0, 0x02);
        self.dom.nodes[idx as usize].text = offset;
        self.dom.parent[idx as usize] = parent;
        self.append(parent, idx);
        idx
    }

    fn append(&mut self, parent: Self::Handle, child: Self::Handle) {
        self.dom.parent[child as usize] = parent;
        self.dom.children.push(child);
        let p = parent as usize;
        if self.dom.children_start[p] == 0 && self.dom.children.len() == 1 {
            self.dom.children_start[p] = (self.dom.children.len() - 1) as u32;
        }
    }

    fn append_before_sibling(&mut self, sibling: Self::Handle, child: Self::Handle) {
        let parent = self.dom.parent[sibling as usize];
        self.dom.parent[child as usize] = parent;
        let p = parent as usize;
        let start = self.dom.children_start[p] as usize;
        let mut pos = start;
        while pos < self.dom.children.len() && self.dom.children[pos] != sibling {
            pos += 1;
        }
        self.dom.children.insert(pos, child);
        for i in 0..self.dom.children_start.len() {
            if self.dom.children_start[i] > pos as u32 {
                self.dom.children_start[i] += 1;
            }
        }
    }

    fn remove_from_parent(&mut self, target: Self::Handle) {
        let parent = self.dom.parent[target as usize];
        if parent == u32::MAX {
            return;
        }
        let p = parent as usize;
        let start = self.dom.children_start[p] as usize;
        let mut pos = start;
        while pos < self.dom.children.len() && self.dom.children[pos] != target {
            pos += 1;
        }
        if pos < self.dom.children.len() {
            self.dom.children.remove(pos);
            for i in 0..self.dom.children_start.len() {
                if self.dom.children_start[i] > pos as u32 {
                    self.dom.children_start[i] -= 1;
                }
            }
        }
        self.dom.parent[target as usize] = u32::MAX;
    }

    fn get_template_contents(&mut self, _target: Self::Handle) -> Self::Handle {
        u32::MAX
    }

    fn same_node(&self, a: &Self::Handle, b: &Self::Handle) -> bool {
        a == b
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> Self::ElemName<'a> {
        let tag_id = self.dom.nodes[*target as usize].tag;
        let name = self.tag_name(tag_id);
        ExpandedName::new(&name)
    }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {
        // Standards mode only
    }

    fn append_doctype_to_document(&mut self, _name: StrTendril, _public_id: Option<StrTendril>, _system_id: Option<StrTendril>) {
        // Ignored
    }

    fn foster_parent_in(&mut self, target: Self::Handle, foster_parent: Self::Handle) {
        self.append(foster_parent, target);
    }
}

pub fn parse_html(html: &str, dom: &mut Dom) {
    use html5ever::parse_document;
    use html5ever::tendril::Tendril;
    use html5ever::ParseOpts;

    let sink = DomTreeSink::new(std::mem::replace(dom, Dom::new()));
    let mut parser = parse_document(sink, ParseOpts::default());
    parser.process(Tendril::from_slice(html.as_bytes()));
    parser.finish();
    println!("Parser finished — nodes written to flat array");
}
