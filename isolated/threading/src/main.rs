extern crate threadpool;

use std::env;
use threadpool::ThreadPool;
use std::sync::mpsc::sync_channel;

pub fn main() {

    // Arguments
    let args: Vec<_> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap_or(44);

    // Pool
    let pool = ThreadPool::default();
    let (tx, rx) = sync_channel::<(u64, u64)>(3);

    // Delegate work
    for k in 0 .. 20 {
        let t = tx.clone();
        pool.execute(move || {
            t.send((k, fib(n))).unwrap();
        });
    }
    println!("work delegates, start reading");

    // Read the output
    for (k, fibk) in rx.iter().take(20) {
        println!("fib #{} is {}", k, fibk);
    }
    pool.join();
    println!("all done");
}

pub fn fib(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}
