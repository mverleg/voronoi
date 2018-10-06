extern crate num_cpus;
extern crate scoped_pool;

use scoped_pool::Pool;
use std::fmt::Debug;
use std::sync::mpsc::sync_channel;
use std::vec::Vec;

/// Transform a collection to another collection using a closure,
/// with execution happening in parallel on a new thread pool with one thread per cpu.
#[allow(dead_code)]
pub fn par_map<I, T, U, F>(collection: I, map: F) -> Vec<U>
where
    I: Iterator<Item = T>,
    T: Send,
    U: Send + Debug,
    F: Send + Copy + Fn(T) -> U,
{
    // Make pool
    let pool = Pool::new(num_cpus::get());

    // Do the mapping
    let results = par_map_on(&pool, collection, map);

    // Stop pool
    pool.shutdown();

    results
}

/// Transform a collection to another collection using a closure,
/// with execution happening in parallel on a given thread pool.
pub fn par_map_on<I, T, U, F>(pool: &Pool, collection: I, map: F) -> Vec<U>
where
    I: Iterator<Item = T>,
    T: Send,
    U: Send + Debug,
    F: Send + Copy + Fn(T) -> U,
{
    // Create the channel to stream output out
    let (tx, rx) = sync_channel::<(usize, U)>(2 * num_cpus::get());

    // Create scope for the output vector.
    pool.scoped(|scope| {
        // Delegate work
        let mut count: usize = 0;
        for (index, value) in collection.enumerate() {
            count += 1;
            let txc = tx.clone();
            scope.execute(move || txc.send((index, map(value))).unwrap());
        }

        // Create an empty vector for the output
        let mut results = Vec::with_capacity(count);
        for _ in 0..count {
            results.push(None);
        }

        // Read and store the output
        for (index, result) in rx.iter().take(count) {
            results[index] = Some(result);
        }

        // Set result on outer scope
        results.into_iter().map(|row| row.unwrap()).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_map() {
        let sq = par_map((0..10).collect::<Vec<i32>>().into_iter(), |x: i32| x * x);
        assert_eq!(vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81,], sq);
    }

    #[test]
    fn test_par_map_closure() {
        let shift = Box::new(10);
        let sq = par_map((0..10).collect::<Vec<i32>>().into_iter(), |x: i32| {
            x * x + *shift
        });
        assert_eq!(vec![10, 11, 14, 19, 26, 35, 46, 59, 74, 91,], sq);
    }
}
