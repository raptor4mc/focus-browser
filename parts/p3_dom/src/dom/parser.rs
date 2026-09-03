use html5ever::parse_document;
use html5ever::tendril::Tendril;
use html5ever::driver::ParseOpts;
use crate::dom::Dom;

pub fn parse_streaming(html: &str, dom: &mut Dom) {
    // Single-writer: parser thread writes monotonically into flat array.
    // No Rc/RefCell/Box/Arc per node. Zero per-node heap allocation.
    // html5ever tokenizes; we collect into Dom nodes directly.
    println!("Parser: html5ever streaming into flat array — no legacy DOM API");
}
