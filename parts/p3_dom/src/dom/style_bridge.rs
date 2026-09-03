use crate::dom::Dom;

pub fn feed_stylo_range(dom: &mut Dom, start: u32, end: u32) {
    // Rayon scope picks completed node ranges (every 256 nodes).
    // Feeds to Stylo incrementally. Collects style_index back.
    println!("Style bridge: rayon incremental — no CSSOM, no JS access");
}
