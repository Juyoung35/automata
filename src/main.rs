fn main() {
    println!("Hello, world!");
}

use ops_derive::OpsDerive;

#[derive(OpsDerive)]
pub struct Deg(isize);