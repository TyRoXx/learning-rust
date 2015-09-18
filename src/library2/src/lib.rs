
pub fn add(first: i32, second: i32) -> i32 {
	first + second
}

#[test]
fn it_works() {
	assert!(5 == add(2, 3));
}
