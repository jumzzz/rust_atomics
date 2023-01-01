use std::thread;
use std::rc::Rc;
use std::sync::Arc;

#[allow(dead_code)]
fn sharing() {
    // static: "owned" by the entire program and not
    // by a single thread
    static X: [i32; 3] = [1, 2, 3];

    let t1 = thread::spawn(|| dbg!(&X));
    let t2 = thread::spawn(|| dbg!(&X));

    t1.join().unwrap();
    t2.join().unwrap();
}

#[allow(dead_code)]
fn box_leak() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));

    t1.join().unwrap();
    t2.join().unwrap();
}

/// Note: Rc is not thread safe. We are aware of this from Rust Book
#[allow(dead_code)]
fn ref_count() {
    let a = Rc::new([1,2,3]);
    let b = a.clone();

    println!("a.as_ptr() = {:?}", a.as_ptr());
    println!("b.as_ptr() = {:?}", b.as_ptr());

    assert_eq!(a.as_ptr(), b.as_ptr());
}

#[allow(dead_code)]
fn atomic_ref_count() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));

    t1.join().unwrap();
    t2.join().unwrap();
}

/// Shadow Variables:
/// Rust allows (and Encourages you) to shadow variables by
/// defining a new variable with a same name.
/// If you do that in the same scope, the orignal variable
/// cannot be named anymore.
#[allow(dead_code)]
fn shadow_ref_count() {
    let a = Arc::new([1, 2, 3]);

    let t1 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    let t2 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    let t3 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}


fn main() {
    // box_leak();
    // sharing();
    // ref_count();
    // atomic_ref_count();
    shadow_ref_count();
}
