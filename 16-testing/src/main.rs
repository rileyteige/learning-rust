extern crate testing;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!")
}

#[test]
fn is_one_equal_to_one() {
	assert_eq!(1i, 1i);
}