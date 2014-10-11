fn main() {
	let x = 5i;

	// Standard 'if' statement, prefer no parens.
	if x == 5i {
		println!("x is five.");
	} else {
		println!("x is not five :(");
	}

	// This is cool, the 'expression' if looks a lot like
	// a ternary expression though.
	let y = if x == 5i { 10i } else { 15i };

	// I wish Rust didn't do the 'expression' if.
	// I only foresee use of it where a ternary would be much more concise.

	println!("Hiding warnings, y is {}.", y)

    println!("Hello, world!")
}
