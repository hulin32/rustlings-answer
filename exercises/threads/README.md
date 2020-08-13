For this exercise check out the [Dining Philosophers example](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html) and the chapter [Concurrency](https://doc.rust-lang.org/book/ch16-01-threads.html) of the Rust Book.



// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM NOT DONE

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        }
    });
    let mut jobs_completed: u32;
    loop {
        jobs_completed = status.lock().unwrap().jobs_completed;
        if jobs_completed < 10 {
            println!("waiting... ({} jobs done)", jobs_completed);
            thread::sleep(Duration::from_millis(500));
        } else {
            break;
        }
    }
}
