pub fn sieve(n: u32) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    
    let mut mark: Vec<bool> = vec![false; n as usize];

    for i in 2..n {
        if mark[i as usize] == false {
            let mut j = 2;
            while i*j < n {
                mark[(i*j) as usize] = true;
                j += 1
            }
        }
    }

    for i in 2..n {
        if mark[i as usize] == false {
            res.push(i);
        }
    }

    return res;
}
