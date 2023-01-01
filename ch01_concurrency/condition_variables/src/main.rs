use std::collections::VecDeque;
use std::thread;
use std::sync::Mutex;
use std::time::Duration;
use std::sync::Condvar;


/// Important Things:
/// - We now not only have a Mutex containing the queue, but also a Condvar 
/// to communicate the "not empty" condition.
/// - We no longer need to know which thread to wakeup, so we don't store
/// the return value from spawn anymore. Instead, we notify the consumer
/// through the condition variable with the notify_one method
/// - Unlocking, waiting, and relocking is all done by wait method.
/// We had to restructure the control flow a bit to be able to pass the
/// guard to the wait method, while still dropping it before processing
/// an item. 
fn main() {

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        // Consuming thread
        s.spawn(|| loop {

            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
            
            drop(q);
            dbg!(item);
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });

}
