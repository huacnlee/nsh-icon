//! The whole catalog, `all` feature (see README, "What it costs").
use nsh_lucide::Icon;
fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "search".into());
    println!("{:?}", Icon::from_name(&name).map(|icon| icon.data_uri().len()));
}
