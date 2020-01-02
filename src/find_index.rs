use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Div, Sub};

pub trait Mid {
    fn midpoint(first: Self, second: Self) -> Self;
}

impl<T> Mid for T
where
    T: Add<Output = T>,
    T: Div<usize, Output = T>,
{
    fn midpoint(first: Self, second: Self) -> Self {
        (first + second) / 2
    }
}

/// Find an index that is equal in an ORDERED range.
/// If there are multiple matches, there is no guarantee about which of them is returned.
/// `f` should return what the order of argument *is*, not what it should be.
/// Order requirements for all x:
/// * if f(x) is Equal, f(x+1) is Equal or Greater
/// * if f(x) is Greater, f(x+1) is Greater
pub fn find_index<T, F>(mut min: T, mut max: T, guess: Option<T>, f: F) -> Option<T>
where
    T: PartialOrd + Add<usize, Output = T> + Sub<usize, Output = T> + Mid + Copy + Debug,
    F: Fn(T) -> Ordering,
{
    assert!(max >= min);
    #[cfg(debug_assertions)]
    {
        // Test the order criterion if in debug mode
        let mut k = min;
        let top = max - 1;
        while k < top {
            let order_current = f(k);
            let order_next = f(k + 1);
            if order_current == Ordering::Equal {
                assert!(order_next == Ordering::Equal || order_next == Ordering::Greater);
            }
            if order_current == Ordering::Greater {
                assert!(order_next == Ordering::Greater);
            }
            k = k + 1;
        }
    }
    // Test the order criterion if in debug mode

    max = max + 1;
    let mid = match guess {
        Some(init) => {
            assert!(init >= min);
            assert!(init <= max);
            init
        }
        None => T::midpoint(min, max),
    };
    // Bisection
    loop {
        match f(mid) {
            Ordering::Less => {
                if mid == min {
                    return None;
                }
                min = mid
            }
            Ordering::Equal => return Some(mid),
            Ordering::Greater => {
                if mid == min {
                    return None;
                }
                max = mid
            }
        };
        let mid = T::midpoint(min, max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_index() {
        let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(Some(6), find_index(0, data.len() - 1, None, |x| data[x].cmp(&6)));
        assert_eq!(Some(0), find_index(0, data.len() - 1, None, |x| data[x].cmp(&0)));
        assert_eq!(
            Some(10),
            find_index(0, data.len() - 1, None, |x| data[x].cmp(&10))
        );
        assert_eq!(None, find_index(0, data.len() - 1, None, |x| data[x].cmp(&-1)));
        assert_eq!(None, find_index(0, data.len() - 1, None, |x| data[x].cmp(&11)));
        let data = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        let needle = find_index(0, data.len() - 1, None, |x| data[x].cmp(&2)).unwrap();
        assert!(needle >= 3 && needle <= 5, "match was {}", needle);
    }
}
