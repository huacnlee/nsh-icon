//! The whole catalog, `all` feature (see README, "What it costs").
use nsh_icon_lucide::Lucide;
fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "search".into());
    println!("{:?}", Lucide::from_name(&name).map(|icon| icon.data_uri().len()));
}
