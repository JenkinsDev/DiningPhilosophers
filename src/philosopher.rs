use std::thread;
use table::Table;

pub struct Philosopher {
    // We use `String` here over `&str`, working with types that own their data is easier
    // than working with one that uses references.
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    pub fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            // To conver our name property to the String struct we have to call .to_string()
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    pub fn eat(&self, table: &Table) {
        // We don't have to unlock, give up control of, the Mutexs below because Rust automagically
        // handles this when we leave the scope!
        //
        // Also, the program can crash if another thread, with these Mutexs, panics!() while we are
        // waiting to gain control of the Mutexs.
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating!", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating!", self.name);
    }
}
