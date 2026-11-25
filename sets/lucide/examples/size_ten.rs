//! Ten icons (see README, "What it costs").
use nsh_icon_lucide::Lucide;
fn main() {
    let icons = [
        Lucide::Search, Lucide::Check, Lucide::X, Lucide::ChevronDown, Lucide::Settings,
        Lucide::Bell, Lucide::House, Lucide::User, Lucide::Trash, Lucide::Plus,
    ];
    let pick = std::env::args().count() % icons.len();
    println!("{}", std::hint::black_box(icons[pick]).data_uri().len());
}
