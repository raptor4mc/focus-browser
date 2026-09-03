use crate::dom::Dom;

pub fn pre_order(dom: &Dom, root: u32, visit: &mut dyn FnMut(u32)) {
    // Sequential traversal for layout/paint. No vtables in hot path.
    println!("Traverse: pre-order sequential — no trait objects in DOM layer");
}
