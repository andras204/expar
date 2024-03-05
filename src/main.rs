use oppar::*;

fn main() {
    let pg = build_paren_tree("( 1 + 2 - ( 2 * 3 ) / ( 2 + 4 ) )".to_string());
    println!("{:?}", pg);
}
