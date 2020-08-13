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
use std::sync::mpsc;

struct JobStatus {
    jobs_completed: u32,
}

// first way
// fn main() {
//     let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
//     let status_shared = Arc::clone(&status);
//     thread::spawn(move || {
//         for _ in 0..10 {
//             thread::sleep(Duration::from_millis(250));
//             let mut status_shared_unwrap = status_shared.lock().unwrap();
//             status_shared_unwrap.jobs_completed += 1;
//         }
//     });
//     loop {
//         let status_unwrap = status.lock().unwrap();
//         if status_unwrap.jobs_completed < 5 {
//             println!("waiting... ");
//             thread::sleep(Duration::from_millis(500));
//         } else {
//             break;
//         }
//     }
// }


// second way
fn main() {
    let jobStatus = JobStatus { jobs_completed: 0 };
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for idx in 0..10 {
            thread::sleep(Duration::from_millis(250));
            tx.send(idx + 1);
        }
    });
    loop {
        let received = rx.recv().unwrap();
        if (received < 10) {
            println!("waiting... ");
            thread::sleep(Duration::from_millis(500));
        } else {
            break;
        }
    }
}