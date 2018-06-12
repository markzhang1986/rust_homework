/// Problem1

/// Computes the sum of all elements in the input i64 slice named `slice`
pub fn sum(slice: &[i64]) -> i64 {
    let mut sum: i64 = 0;
    for ele in slice {
        sum += ele;
    }
    return sum;
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i64>) -> Vec<i64> {
    let mut ret: Vec<i64> = Vec::new();
    for ele in vs {
        if !ret.contains(ele) {
            ret.push(*ele);
        }
    }
    return ret;
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i64` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i64>, pred: &Fn(i64) -> bool) -> Vec<i64> {
    let mut ret: Vec<i64> = Vec::new();
    for ele in vs {
        if pred(*ele) {
            ret.push(*ele);
        }
    }
    return ret;
}
