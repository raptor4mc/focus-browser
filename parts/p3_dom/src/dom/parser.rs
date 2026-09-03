use html5ever::tree_builder::{TreeSink, QuirksMode, ElementFlags, NodeOrText};
use html5ever::{QualName, Attribute};
use html5ever::tendril::StrTendril;
use markup5ever::{ExpandedName, Namespace};

pub struct DomParser {
    pub dom: super::Dom,
    temp_children: Vec<Vec<u32>>,
    html_ns: Namespace,
}

impl DomParser {
    pub fn new(dom: super::Dom) -> Self {
        Self {
            dom,
            temp_children: Vec::new(),
            html_ns: Namespace::from("http://www.w3.org/1999/xhtml"),
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

    fn parse_error(&mut self, _msg: std::borrow::Cow<'static, str>) {}

    fn get_document(&mut self) -> u32 { 0 }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        let node = &self.dom.nodes[*target as usize];
        ExpandedName {
            ns: &self.html_ns,
            local: &self.dom.tag_names[node.tag as usize],
        }
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>, _flags: ElementFlags) -> u32 {
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

    fn append_before_sibling(&mut self, _sibling: &Self::Handle, _new_node: NodeOrText<Self::Handle>) {}

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        let idx = *target as usize;
        if idx < self.dom.parent.len() {
            self.dom.parent[idx] = u32::MAX;
        }
    }

    fn reparent_children(&mut self, _node: &Self::Handle, _new_parent: &Self::Handle) {}

    fn mark_script_already_started(&mut self, _node: &Self::Handle) {}

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {}

    fn append_doctype_to_document(&mut self, _name: StrTendril, _public_id: StrTendril, _system_id: StrTendril) {}

    fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
        u32::MAX
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn add_attrs_if_missing(&mut self, _target: &Self::Handle, _attrs: Vec<Attribute>) {}
}
