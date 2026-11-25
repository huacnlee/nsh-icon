//! Size baseline: the same program with no icon (see README, "What it costs").
fn main() {
    let name = std::env::args().nth(1).unwrap_or_default();
    println!("{}", std::hint::black_box(name).len());
}
