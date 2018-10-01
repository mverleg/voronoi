extern crate scoped_pool;
extern crate num_cpus;

use scoped_pool::Pool;

use std::env;
use std::sync::mpsc::sync_channel;

pub fn main() {

    // Arguments
    let args: Vec<_> = env::args().collect();
    let cnt = Box::new(if args.len() >= 2 {
        args[1].parse::<u64>().unwrap_or(44)
    } else {
        44
    });

    // Pool
    let pool = Pool::new(num_cpus::get());
    let (tx, rx) = sync_channel::<(u64, u64)>(3);

    pool.scoped(|scope| {
        // Delegate work
        for k in 0 .. 20 {
            let t = tx.clone();
            let n = &cnt;
            scope.execute(move ||
                t.send((k, fib(&n))).unwrap()
            );
        }
        // Read the output
        for (k, fibk) in rx.iter().take(20) {
            println!("fib #{} is {}", k, fibk);
        }
    });
    println!("work delegates, start reading");

    pool.shutdown();
    println!("all done");
}

pub fn fib(n: &u64) -> u64 {
    if n <= &2 {
        return 1;
    }
    fib(&(n - 1)) + fib(&(n - 2))
}
