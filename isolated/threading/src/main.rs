extern crate threadpool;

use std::env;
use threadpool::ThreadPool;

pub fn main() {
    let pool = ThreadPool::new(5);
    let args: Vec<_> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap_or(45);
    println!("fib #{} is {}", n, fib(n));
}

pub fn fib(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}
