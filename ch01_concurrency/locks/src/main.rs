/// RwLock or reader-writer lock is the concurrent version of RefCell
use std::marker::PhantomData;
use std::cell::Cell;
// use std::rc::Rc;
// use std::thread;

/// Thread Safety Types: Send and Sync
/// Send
/// A type is Send if it can be sent to another thread.
/// Sync
/// A type is Sync if it can be shared to another thread.

struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>
}

unsafe impl Send for X {}
unsafe impl Sync for X {}

/// This triggers a compiler error
/// commented out by default
// fn sending_non_send_types() {
    // let a = Rc::new(123);
    // thread::spawn(move || {
    //     dbg!(a);
    // });
// }

fn main() {
}
