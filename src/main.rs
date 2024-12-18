fn main() {
    println!("Hello, world!");
}

use ops_derive::OpsDerive;

#[derive(OpsDerive)]
#[ops_derive(
    convert(())
)]
pub struct Deg(isize);