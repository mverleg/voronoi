use std::cmp::Ordering;
use std::ops::{Add, Sub, Div};

/// Find an index that is equal in an ORDERED range.
/// If there are multiple matches, there is no guarantee about which of them is returned.
/// Order requirements for all x:
/// * if f(x) is Equal, f(x+1) is Equal or Greater
/// * if f(x) is Greater, f(x+1) is Greater
pub fn find_index<T, F>(mut min: T, mut max: T, f: F) -> Option<T>
    where T: PartialOrd + Add<usize, Output=T> + Add<T, Output=T> + Sub<usize, Output=T> + Sub<T, Output=T> + Div<usize, Output=T> + Copy,
          F: Fn(T) -> Ordering
{
    assert!(max >= min);
    // Test the order criterion if in debug mode
    let mut k = min;
    let top = max - 1;
    while k < top {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_index() {
        let data: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(Some(6), find_index(0, data.len() - 1, |x| data[x].cmp(&6)));
        assert_eq!(Some(0), find_index(0, data.len() - 1, |x| data[x].cmp(&0)));
        assert_eq!(Some(10), find_index(0, data.len() - 1, |x| data[x].cmp(&10)));
        assert_eq!(None, find_index(0, data.len() - 1, |x| data[x].cmp(&-1)));
        assert_eq!(None, find_index(0, data.len() - 1, |x| data[x].cmp(&11)));
        let data = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        let needle = find_index(0, data.len() - 1, |x| data[x].cmp(&2)).unwrap();
        assert!(needle >= 3 && needle <= 5, "match was {}", needle);
    }
}
