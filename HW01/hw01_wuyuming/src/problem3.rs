/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`

pub fn sieve(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    /* Init, remove 0 and 1 */
    for i in 2..n {
        vec.push(i);
    }

    
    let mut i: usize = 0;

    while i < vec.len() {
        let mut j: usize = i + 1;
        while j < vec.len() {
            if vec[j] % vec[i] == 0 {
                vec.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    return vec;
}

