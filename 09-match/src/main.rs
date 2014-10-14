fn main() {
    let x = 5i;

    match x {
    	1 => println!("one"),
    	2 => println!("two"),
    	3 => println!("three"),
    	4 => println!("four"),
    	5 => println!("five"),
    	_ => println!("something else")
    }

    let y = 10i;

    match cmp(x, y) {
    	Less => println!("less"),
    	Greater => println!("greater"),
    	Equal => println!("equal")
    }
}

fn cmp(x: int, y: int) -> Ordering {
	if x < y { Less }
	else if x > y { Greater }
	else { Equal }
}