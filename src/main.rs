fn main() {
    let a = bit_to_traits(ADD);
    println!("{a:?}");
    println!("Hello, world!");
}

use ops_derive::OpsDerive;

// #[derive(OpsDerive)]
// #[ops_derive(
//     convert(())
// )]
// pub struct Deg(isize);

use ops_derive_consts::*;