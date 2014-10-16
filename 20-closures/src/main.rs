fn main() {
    let add_one = |x| { 1i + x };

    println!("The sum of 5 plus 1 is {}", add_one(5i));

    let x = 5i;

    let printer = || { println!("x is: {}", x); };

    printer();

    let p = proc() { x * x };
    println!("{}", p());

    let square = |x: int| { x * x };

    assert_eq!(50i, twice(5i, square));
    assert_eq!(50i, twice(5i, |x: int| { x * x }));
    assert_eq!(50i, twice(5i, named_square));
}

fn twice(x: int, f: |int| -> int) -> int {
    f(x) + f(x)
}

fn named_square(x: int) -> int {
    x * x
}