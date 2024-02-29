use std::path::Path;
use wit_parser::UnresolvedPackage;

#[test]
fn ready() {
    println!("it works!")
}

fn prepare() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = include_str!("../../preview2/http/types.wit");
    let package = UnresolvedPackage::parse(here, file).unwrap();
}
