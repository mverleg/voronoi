use std::cmp::Ordering;
use std::ops::{Add, Sub, Div};
use std::ops::Range;

/// Find an index that is equal in an ORDERED range.
/// If there are multiple matches, there is no guarantee about which of them is returned.
/// Order requirements for all x:
/// * if f(x) is Equal, f(x+1) is Equal or Greater
/// * if f(x) is Greater, f(x+1) is Greater
fn find_index<T, F>(mut min: T, mut max: T, f: F) -> Option<T>
    where T: PartialOrd + Add<usize, Output=T> + Add<T, Output=T> + Sub<usize, Output=T> + Sub<T, Output=T> + Div<usize, Output=T> + Copy,
          F: Fn(T) -> Ordering
{
    assert!(max >= min);
    // Test the order criterion if in debug mode
    let mut k = min;
    while k < max - 1 {
        if f(k) == Ordering::Equal {
            debug_assert!(f(k+1) == Ordering::Equal || f(k+1) == Ordering::Greater);
        }
        if f(k) == Ordering::Greater {
            debug_assert!(f(k+1) == Ordering::Greater);
        }
        k = k + 1;
    }
    max = max + 1;
    // Bisection
    loop {
        let mid = (min + max) / 2;
        match f(mid) {
            Ordering::Less => {
                if mid == min {
                    return None;
                }
                min = mid
            },
            Ordering::Equal => return Some(mid),
            Ordering::Greater => {
                if mid == min {
                    return None;
                }
                max = mid
            },
        }
    }
}
