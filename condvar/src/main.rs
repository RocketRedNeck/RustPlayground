
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time;


fn main() {
let pair = Arc::new((Mutex::new(0u32), Condvar::new()));
let pair2 = Arc::clone(&pair);

// Inside of our lock, spawn a new thread, and then wait for it to start.
thread::spawn(move|| {
    let (lock, cvar) = &*pair2;
    let delay_millis = time::Duration::from_millis(10);
    loop {
        thread::sleep(delay_millis);
        {
            // Scope to ensure lock is released before notify
            let mut started = lock.lock().unwrap();
            *started += 1;
            println!{"thread at {}", *started};
        }
        // We notify the condvar that the value has changed.
        cvar.notify_all();
    }
});

// Wait for the thread to start up.
let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
while *started < 10 {
    println!{"At {}. Waiting...", *started};    
    started = cvar.wait(started).unwrap();
    println!{"received at {}", *started};    
}
}
