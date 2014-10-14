fn main() {
    println!("Hello, world!")

    tuples();
    structs();
    tuple_structs_new_types();
    enums();
}

fn tuples() {
	let x = (1i, "hello");
    let x2: (int, &str) = (1, "hello");
    println!("{}", x);
    println!("{}", x2);

    let (a, b, c) = (1i, 2i, 3i); // Pattern matching
    println!("a is {}, b is {}, c is {}.", a, b, c);

    let mut left = (1i, 2i);
    let right = (2i, 3i);
    left = right; // Assignment of one tuple into another, if they share types.
    println!("Left is {}.", left);

    let x3 = (1i, 2i, 3i);
    let y3 = (2i, 2i, 4i);
    if x3 == y3 {
    	println!("yes!");
    } else {
    	println!("no!");
    }

    let (x4, y4) = next_two(5i);
    println!("x = {}, y = {}", x4, y4);
}

fn next_two(x: int) -> (int, int) {
	(x + 1, x + 2)
}

fn structs() {
	let origin = Point { x: 0, y: 0 };
	println!("The origin is at ({}, {}).", origin.x, origin.y);

	let mut mutable_point = Point { x: 0, y: 0 };
	mutable_point.x = 5;
	println!("The mutable_point is at ({}, {}).", mutable_point.x, mutable_point.y);
}

struct Point {
	x: int,
	y: int
}

fn tuple_structs_new_types() {
	let black = Color(0, 0, 0);
	let origin = Point3d(0, 0, 0);

	let length = Inches(10); // Cool way of doing "type synonyms" (in essence).
	let Inches(integer_length) = length;
	println!("The length is {} inches.", integer_length);
}

struct Color(int, int, int);
struct Point3d(int, int, int);
struct Inches(int);

fn enums() {
	let x = 5i;
	let y = 10i;

	let ordering = cmp(x, y);

	if ordering == Less {
		println!("Less");
	} else if ordering == Greater {
		println!("Greater");
	} else {
		println!("Equal");
	}

	let optional_x = Value(5);
	let optional_y = Missing;

	match optional_x {
		Value(n) => println!("x is {:d}", n),
		Missing  => println!("x is missing!")
	}

	match optional_y {
		Value(n) => println!("y is {:d}", n),
		Missing  => println!("y is missing!")
	}
}

// Provided by the standard library:
// 	enum Ordering {
// 		Less,
//		Equal,
//		Greater
// 	}

fn cmp(a: int, b: int) -> Ordering {
	if a < b { Less }
	else if a > b { Greater }
	else { Equal }
}

enum OptionalInt {
	Value(int),
	Missing
}