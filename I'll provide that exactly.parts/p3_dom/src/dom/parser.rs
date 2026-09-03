use html5ever::tree_builder::{TreeSink, QuirksMode, ElementFlags, NodeOrText};
use html5ever::{QualName, Attribute};
use html5ever::tendril::{StrTendril, TendrilSink};
use markup5ever::{ExpandedName, ns};
use std::collections::HashMap;

use crate::dom::{Dom, Node, NodeFlags};

pub struct DomParser {
    pub dom: Dom,
    temp_children: Vec<Vec<u32>>,
}

impl DomParser {
    pub fn new(dom: Dom) -> Self {
        Self {
            dom,
            temp_children: Vec::new(),
        }
    }

    fn make_text_node(&mut self, text: StrTendril) -> u32 {
        let text_offset = self.dom.string_arena.len() as u32;
        self.dom.string_arena.extend_from_slice(text.as_bytes());
        let node_idx = self.dom.nodes.len() as u32;
        self.dom.nodes.push(super::Node {
            tag: 0,
            attrs: 0,
            text: text_offset,
            flags: super::NodeFlags::IS_TEXT,
            style_index: 0,
            layout_index: 0,
            _pad: 0,
        });
        self.dom.parent.push(u32::MAX);
        self.temp_children.push(Vec::new());
        node_idx
    }
}

impl TreeSink for DomParser {
    type Handle = u32;
    type Output = Self;

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>, _flags: ElementFlags) -> Self::Handle {
        let tag_str = name.local.as_ref();
        let tag_id = self.dom.intern_tag(&tag_str);
        let node_idx = self.dom.nodes.len() as u32;

        let attr_offset = self.dom.store_attrs(&attrs);

        self.dom.nodes.push(super::Node {
            tag: tag_id,
            attrs: attr_offset,
            text: 0,
            flags: super::NodeFlags::IS_ELEMENT,
            style_index: 0,
            layout_index: 0,
            _pad: 0,
        });

        self.dom.parent.push(u32::MAX);
        self.temp_children.push(Vec::new());

        node_idx
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(node) => {
                let p = *parent as usize;
                let c = node as usize;
                if p < self.temp_children.len() {
                    self.temp_children[p].push(node);
                }
                if c < self.dom.parent.len() {
                    self.dom.parent[c] = *parent;
                }
            }
            NodeOrText::AppendText(text) => {
                let text_node = self.make_text_node(text);
                self.append(parent, NodeOrText::AppendNode(text_node));
            }
        }
    }

    fn append_before_sibling(&mut self, _sibling: &Self::Handle, _new_node: NodeOrText<Self::Handle>) {
        // TODO
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        let idx = *target as usize;
        if idx < self.dom.parent.len() {
            self.dom.parent[idx] = u32::MAX;
        }
    }

    fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
        u32::MAX
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        let tag_id = self.dom.nodes[*target as usize].tag;
        ExpandedName {
            ns: &ns!(html),
            local: &self.dom.tag_names[tag_id as usize],
        }
    }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {
        // Standards mode only
    }

    fn append_doctype_to_document(&mut self, _name: StrTendril, _public_id: StrTendril, _system_id: StrTendril) {
        // Ignored
    }

    fn finish(mut self) -> Self::Output {
        self.dom.children_start.clear();
        self.dom.children.clear();
        let mut offset = 0u32;
        for node_children in &self.temp_children {
            self.dom.children_start.push(offset);
            for &child in node_children {
                self.dom.children.push(child);
            }
            offset += node_children.len() as u32;
        }
        self.dom.children_start.push(offset);
        self
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
}

pub fn parse_html(html: &str, dom: &mut Dom) {
    use html5ever::parse_document;
    use html5ever::ParseOpts;

    let sink = DomParser::new(std::mem::replace(dom, Dom::new()));
    let mut parser = parse_document(sink, ParseOpts::default());
    parser.process(StrTendril::from_slice(html));
    parser.finish();
    println!("Parser finished — nodes written to flat array");
}
