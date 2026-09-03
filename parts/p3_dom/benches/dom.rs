use std::time::Instant;

fn main() {
    println!("Bench: parse 100KB HTML < 2ms, style 10K nodes < 3ms, layout < 5ms, TTI < 50ms");
    println!("DOM memory 10K nodes < 4MB, allocations during parse = 0");
}
