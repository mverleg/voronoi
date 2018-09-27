#![feature(duration_as_u128)]

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

pub fn main() {
    let mut rng = rand::thread_rng();
    let mut data_4_deriv: Vec<Deriv> = (0 .. 100000)
        .map(|_| Deriv { x: rng.gen_range(0, 5000)})
        .collect();
    let mut data_4_manual: Vec<Manual> = data_4_deriv.iter()
        .map(|d| Manual { x: d.x })
        .collect();
    let start = Instant::now();
    bubble_sort(&mut data_4_deriv);
    println!("derive: {:.3}ms", Instant::now().duration_since(start).as_millis());
    let start = Instant::now();
    bubble_sort(&mut data_4_manual);
    println!("derive: {:.3}ms", Instant::now().duration_since(start).as_millis());
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Deriv {
    x: usize,
}

#[derive(Eq, PartialEq)]
struct Manual {
    x: usize,
}

impl PartialOrd for Manual {
    #[inline]
    fn partial_cmp(&self, other: &Manual) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Manual {
    #[inline]
    fn cmp(&self, other: &Manual) -> Ordering {
        self.x.cmp(&other.x)
    }
}

// https://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort#Rust
fn bubble_sort<T: Ord>(values: &mut[T]) {
    let mut n = values.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..n {
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }

        n = n - 1;
    }
}
