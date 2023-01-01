use std::cell::{Cell, RefCell};

#[allow(dead_code)]
fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;

    let after = *a;

    if before != after {
        println!("wow!");
    }
}


#[allow(dead_code)]
fn push_to_cell(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

#[allow(dead_code)]
fn push_to_ref_cell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}


#[allow(dead_code)]
fn using_cells() {
    let v1 = Cell::new(vec![1, 2, 3]);
    let v2 = RefCell::new(vec![1, 2, 3]);

    push_to_cell(&v1);
    push_to_ref_cell(&v2);

    println!("v1 = {:?}", v1.take());
    println!("v2 = {:?}", v2.take());
}




/// Safety
/// When calling any unsafe function, read its documentation
/// carefully and make sure you fully understand its safety
/// requirements: the assumptions you need to uphold, as
/// the caller, to avoid undefined behavior.
fn main() {
    // f();
    using_cells();
}
