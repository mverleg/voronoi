

extern crate scoped_pool;
extern crate num_cpus;

use scoped_pool::Pool;

use std::env;
use std::sync::mpsc::sync_channel;
use std::vec::Vec;

/// Transform a collection to another collection using a closure,
/// with execution happening in parallel on a new thread pool with one thread per cpu.
pub fn par_map<T>(collection: i32, map: i32 {

    // Make pool
    let pool = Pool::new(num_cpus::get());

    // Do the mapping
    let result = par_map_on(&pool, collection, map);

    // Stop pool
    pool.shutdown();

    result
}

/// Transform a collection to another collection using a closure,
/// with execution happening in parallel on a given thread pool.
pub fn par_map_on<T, U>(pool: &Pool, collection: i32, map: i32) -> Vec<U> {

    // Create the channel to stream output out
    let (tx, rx) = sync_channel::<(u64, U)>(3);

    pool.scoped(|scope| {

        // Delegate work
        let mut count: usize = 0;
        for (index, value) in collection.enumerate() {

            count += 1;
            scope.execute(move ||
                tx.send((
                    index,
                    map(value),
                ))?
            );
        }

        // Read the output
        for (index, result) in rx.iter().take(count) {
            println!("fib #{} is {:?}", index, result);
        }
    });

    Vec::<U>::new()  //TODO @mark: THIS CODE IS TEMPORARY!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_map() {
        let sq = par_map((0 .. 10).into_iter(), |x: i32| x*x);
        assert_eq!(vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81,], sq);

    }
}
