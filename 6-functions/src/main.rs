fn main() {
    println!("Hello, world!")
    foo();
    print_number(5);
    print_sum(8, 14);
    println!("5 + 1 is {}", add_one(5));
    println!("foo_with_early_return of 3 is {}", foo_with_early_return(3));
}

fn foo() {
}

fn print_number(x: int) {
	println!("x is: {}", x);
}

fn print_sum(x: int, y: int) {
	println!("sum is: {}", x + y);
}

// Note here the lack of a 'return' keyword, as well
// as the lack of a semicolon.
fn add_one(x: int) -> int {
	x + 1
}

fn foo_with_early_return(x: int) -> int {
	if x < 5 { return x; }

	x + 1
}