use std::sync::Mutex;
use std::thread;
use std::time::Duration;


#[allow(dead_code)]
fn mutex_v1() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
         s.spawn(|| {
            let mut guard = n.lock().unwrap();
            for _ in 0..100 {
                *guard += 1;
            }
         }); 
        }
    });

    // into_inner() method takes the ownership of the mutex,
    // which guarantees that nothing else can have a reference
    // to the mutex anymore, making locking unnecessary.
    assert_eq!(n.into_inner().unwrap(), 1000);
}

#[allow(dead_code)]
fn mutex_v2() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
         s.spawn(|| {
            let mut guard = n.lock().unwrap();
            for _ in 0..100 {
                *guard += 1;
            }
            let id = thread::current().id();
            println!("current_id = {id:?}");

            drop(guard);
            thread::sleep(Duration::from_secs(1));

         }); 
        }
    });

    // into_inner() method takes the ownership of the mutex,
    // which guarantees that nothing else can have a reference
    // to the mutex anymore, making locking unnecessary.

    let val = n.into_inner().unwrap();
    println!("val = {val}");
    
    assert_eq!(val, 1000);
}
 
#[allow(dead_code)]
fn accidental_longer_lock() {
    let vals = Mutex::new(vec![1, 2, 3, 4, 5, 6]);
    
    println!("accidental lock version: ");
    
    thread::scope(|s| {
        for _ in 0..6 {
            s.spawn(|| {
                
                // In this case, the lock is retained until the end
                // of the scope since if let Some(item) waits for
                // the scope to end until some assignmnet happens
                if let Some(item) = vals.lock().unwrap().pop() {
                    println!("val = {item}");
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }
    });
}

#[allow(dead_code)]
fn shorter_lock() {
    let vals = Mutex::new(vec![1, 2, 3, 4, 5, 6]);

    println!("shorter lock version: ");
    
    thread::scope(|s| {
        for _ in 0..6 {
            s.spawn(|| {

                let item = vals.lock().unwrap().pop();

                if let Some(item) = item {
                    println!("val = {item}");
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }
    });
}

fn main() {
    // mutex_v1();
    // mutex_v2();
    // accidental_longer_lock();
    shorter_lock();
}
