/*
language threads -> os threads
Execution order of multiple execution threads are not deterministic.
Dead lock condition.
Race condition.

Thread implementation types:

- 1:1: build in in rust. 
- green thread N language threads : M os threads.

*/

use std::{thread, time::Duration, sync::mpsc::Sender};

pub fn check_thread() -> thread::JoinHandle<()>{
    
    return thread::spawn( || {
        for i in 1..10{
            println!("Hi thread no {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}


pub fn process_url(url: String, tx: Sender<String>) {
    thread::sleep(Duration::from_millis(1)); // simulating IO 
    tx.send(url.to_uppercase()); // send returns a result make sure we handle it during production.
}
