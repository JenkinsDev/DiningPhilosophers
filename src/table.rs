use std::sync::Mutex;

pub struct Table {
    // A Mutex is a way to controll concurrency; only one thread can access the contents at once.
    pub forks: Vec<Mutex<()>>,
}
