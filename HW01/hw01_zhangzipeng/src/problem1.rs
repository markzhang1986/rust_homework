pub fn sum(slice: &[i32]) -> i32 {
    let mut res: i32 = 0;

    for n in slice {
        res += *n; 
    }

    res
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    
    for n in vs {
        if res.contains(n) {

        } else {
            res.push(*n);
        }
    }

    res
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut res = Vec::new();

    for n in vs {
        if pred(*n) {
            res.push(*n);
        }
    }

    res
}
