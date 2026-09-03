use html5ever::tree_builder::{TreeSink, QuirksMode, ElementFlags, NodeOrText};
use html5ever::{Attribute, QualName};
use html5ever::tendril::{StrTendril, TendrilSink};
use markup5ever::interface::tree_builder::ExpandedName;
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
        for (k, v) in &self.tag_map {
            if *v == id {
                return k.clone();
            }
        }
        "unknown".to_string()
    }
}

impl TreeSink for DomTreeSink {
    type Handle = u32;
    type Output = u32;

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>, _flags: ElementFlags) -> Self::Handle {
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

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        let child_idx = match child {
            NodeOrText::Element(h) => h,
            NodeOrText::Text(h) => h,
        };
        self.dom.parent[child_idx as usize] = *parent;
        self.dom.children.push(child_idx);
        let p = *parent as usize;
        if self.dom.children_start[p] == 0 && self.dom.children.len() == 1 {
            self.dom.children_start[p] = (self.dom.children.len() - 1) as u32;
        }
    }

    fn append_before_sibling(&mut self, sibling: &Self::Handle, child: NodeOrText<Self::Handle>) {
        let parent = self.dom.parent[*sibling as usize];
        let child_idx = match child {
            NodeOrText::Element(h) => h,
            NodeOrText::Text(h) => h,
        };
        self.dom.parent[child_idx as usize] = parent;
        let p = parent as usize;
        let start = self.dom.children_start[p] as usize;
        let mut pos = start;
        while pos < self.dom.children.len() && self.dom.children[pos] != *sibling {
            pos += 1;
        }
        self.dom.children.insert(pos, child_idx);
        for i in 0..self.dom.children_start.len() {
            if self.dom.children_start[i] > pos as u32 {
                self.dom.children_start[i] += 1;
            }
        }
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        let parent = self.dom.parent[*target as usize];
        if parent == u32::MAX {
            return;
        }
        let p = parent as usize;
        let start = self.dom.children_start[p] as usize;
        let mut pos = start;
        while pos < self.dom.children.len() && self.dom.children[pos] != *target {
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
        self.dom.parent[*target as usize] = u32::MAX;
    }

    fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
        u32::MAX
    }

    fn same_node(&self, a: &Self::Handle, b: &Self::Handle) -> bool {
        a == b
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        let tag_id = self.dom.nodes[*target as usize].tag;
        let name = self.tag_name(tag_id);
        ExpandedName { local: name.into(), ns: "".into() }
    }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {
        // Standards mode only
    }

    fn append_doctype_to_document(&mut self, _name: StrTendril, _public_id: StrTendril, _system_id: StrTendril) {
        // Ignored
    }

    fn finish(self) -> Self::Output {
        0
    }

    fn parse_error(&mut self, _msg: std::borrow::Cow<'static, str>) {
        // Ignore parse errors for greenfield
    }

    fn get_document(&mut self) -> Self::Handle {
        0
    }

    fn create_comment(&mut self, _text: StrTendril) -> Self::Handle {
        0
    }

    fn create_pi(&mut self, _target: StrTendril, _data: StrTendril) -> Self::Handle {
        0
    }

    fn append_based_on_parent_node(&mut self, _parent: &Self::Handle, _prev: &Self::Handle, _child: NodeOrText<Self::Handle>) {
        // Minimal
    }

    fn add_attrs_if_missing(&mut self, _target: &Self::Handle, _attrs: Vec<Attribute>) {
        // Minimal
    }

    fn reparent_children(&mut self, _old_parent: &Self::Handle, _new_parent: &Self::Handle) {
        // Minimal
    }

    fn output(&self) -> &Self::Output {
        &0
    }
}

pub fn parse_html(html: &str, dom: &mut Dom) {
    use html5ever::parse_document;
    use html5ever::ParseOpts;

    let sink = DomTreeSink::new(std::mem::replace(dom, Dom::new()));
    let mut parser = parse_document(sink, ParseOpts::default());
    parser.process(StrTendril::from_slice(html.as_bytes()));
    parser.finish();
    println!("Parser finished — nodes written to flat array");
}
