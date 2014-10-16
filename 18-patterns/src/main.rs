fn main() {
	let x = 1i;

	match x {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		4 => println!("four"),
		_ => println!("anything")
	}

	let x2 = 2i;

	match x2 {
		1 | 2 => println!("one or two"),
		3 => println!("three"),
		_ => println!("anything")
	}

	let x3 = 3i;

	match x3 {
		1 .. 5 => println!("one through five"),
		_ => println!("anything")
	}

	let x4 = 3i;

	match x4 {
		x @ 1 .. 5 => println!("got {}", x),
		_ => println!("anything")
	}

	let x5 = Value(5i);

	match x5 {
		Value(..) => println!("Got an int!"),
		Missing => println!("No such luck.")
	}

	let x6 = Value(6i);

	match x6 {
		Value(x) if x > 5 => println!("Got an int greater than five!"),
		Value(..) => println!("Got an int!"),
		Missing => println!("No such luck.")
	}

	let x7 = &5i;

	match x7 {
		&x => println!("Got a value: {}", x)
	}

	let x8 = 5i;

	match x8 {
		ref x => println!("Got a reference to {}", x)
	}

	let mut x9 = 5i;

	match x9 {
		ref mut x => println!("Got a mutable reference to {}", x)
	}

	let origin = Point { x: 0i, y: 0i };

	match origin {
		Point { x: x, y: y } => println!("({},{})", x, y)
	}

	match origin {
		Point { x: x, .. } => println!("x is {}", x)
	}

	// Match patterns can get complex as well:
	//
	//	match x {
    //		Foo { x: Some(ref name), y: None } => ...
	//	}
}

enum OptionalInt {
	Value(int),
	Missing
}

struct Point {
	x: int,
	y: int
}