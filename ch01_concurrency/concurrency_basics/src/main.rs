use std::thread;

#[allow(dead_code)]
fn f() {

    println!("Hello from another thread.");
    let id = thread::current().id();
    println!("thread_id = {id:?}");
}

#[allow(dead_code)]
fn basic_threads() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("This is the main thread!");
    let id = thread::current().id();
    println!("thread_id = {id:?}");

    t1.join().unwrap();
    t2.join().unwrap();    
}

#[allow(dead_code)]
fn threads_closure_v1() {

    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();

}

#[allow(dead_code)]
fn threads_closure_v2() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}

/// std::thread::scope
/// The Rust standard library provides the std::thread::scope function
/// to spawn such scoped threads. It allows us to spawn threads that
/// cannot outlive the scope of the closure we pass to that function,
/// making it possible to safely borrow local variables.
#[allow(dead_code)]
fn thread_scope() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length = {}", numbers.len());
            let id = thread::current().id();
            println!("first_scope thread_id = {id:?}");
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
            let id = thread::current().id();
            println!("second_scope thread_id = {id:?}");
        });
    })
}

fn main() {
    // basic_threads();
    // threads_closure_v1();
    // threads_closure_v2();
    thread_scope();
}

