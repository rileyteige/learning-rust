use std::rc::Rc;

fn main() {
    // Simple reference
    let x = 5i;
    let y = &x;

    assert_eq!(5i, *y);

    assert_eq!(6, add_one(y));

    // Multiple aliased references
    let z = &x;

    let mut x2 = 5i;

    // Allocate a variable on the heap. Will automatically be freed.
    let mut x3 = box 5i;

    assert_eq!(5i, *x3);

    rc_and_arc();
}

fn add_one(x: &int) -> int { *x + 1 }

fn rc_and_arc() {
    let x = Rc::new(5i);

    // How to use Arc<T>?

    assert_eq!(5i, *x);
}
