mod philosopher;
mod table;

use std::thread;
use std::sync::{Arc,Mutex};
use philosopher::Philosopher;
use table::Table;

fn main() {
    // Arc stands for `Atomic Reference Count`, and we need it to share our Table across multiple
    // threads.  As we share this variable, the reference count will go up, and when each thread ends
    // it will go down.  @see `let table = table.clone();` within our .map() call.
    let table = Arc::new(Table { forks: vec! [
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // `.clone()` is a method on the Arc struct.  We use `.clone()` to increment the reference
        // counter so Rust knows how to correctly deallocate uses of `table` across threads.
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    // After we set up the thread::spawn(move || {}) expression, we can no longer access the
    // `vars` that are moved.  This is because this thread no longer has access over the variable
    // and the new thread has access to it.
    // for p in &philosophers {
    //     p.eat();
    // }

    for h in handles {
        // .join() blocks execution until the thread has completed execution.  This ensures that the
        // threads complete their work before the program exits.
        h.join().unwrap();
    }

    // Because of our .join() calls, ^^^ up there, this println!() won't get called until all threads
    // are finished.  This is because we have blocked the main thread until executing until the
    // background threads have finished.
    println!("Test");
}
