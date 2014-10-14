fn main() {
    // &str versus String.
    // &str = 'string slice'.
    // 'String slices have a fixed size, and cannot be mutated.'

    // type will be &str.
    let string = "Hello there.";

    // type will be String.
    let mut s = "Hello".to_string();
    println!("{}", s);

    s.push_str(", world!");
    println!("{}", s);

    takes_slice(s.as_slice());

    // Prefer 'as_slice()' over 'to_string()' when
    // comparing against a literal string.
}

fn takes_slice(slice: &str) {
	println!("Got: {}", slice);
}
