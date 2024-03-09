use oppar::*;

fn main() {
    let original = "( 1 + 2 - ( 2 * 3 () () (() ()) ) / ( 2 + 4 ) )";
    println!("{}", original);
    let pg = build_paren_tree(original).unwrap();
    println!("{}", pg);
}
