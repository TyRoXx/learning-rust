extern crate library2;
use library2::add;

fn main() {
    println!("Hello, world!");
}

#[test]
fn trivial() {
	assert!(5 == add(2, 3));
}
