fn main() {
    let a = bit_to_traits(ADD);
    println!("{a:?}");
    println!("Hello, world!");
}

use ops_derive::*;

// #[derive(OpsDerive)]
// #[ops_derive(
//     convert(())
// )]
// pub struct Deg(isize);