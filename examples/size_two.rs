//! Two icons (see README, "What it costs").
use nsh_lucide::Icon;
fn main() {
    let icons = [Icon::Search, Icon::Check];
    let pick = std::env::args().count() % icons.len();
    println!("{}", std::hint::black_box(icons[pick]).data_uri().len());
}
