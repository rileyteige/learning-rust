fn main() {
	for_loop();
	while_loop();
}

fn for_loop() {
	for x in range(0i, 10i) {
    	println!("{:d}", x);
    }
}

fn while_loop() {
	let mut x = 5u;
	let mut done = false;

	while !done {
		x += x - 3;
		println!("{}", x);
		if x % 5 == 0 { done = true }
	}

	// Can also do this for an infinite loop (preferred):
	// 	loop {
	// 		-- loop body here
	// 	}

	x = 5u;
	loop {
		x += x - 3;
		println!("{}", x);
		if x % 5 == 0 { break; }
	}

	for x in range(0i, 10i) {
		if x % 2 == 0 { continue; }

		println!("{:d}", x);
	}
}