#[derive(Clone, Copy, Debug, Eq, PartialEq)]

pub enum Peg {
    A,
    B,
    C,
}

pub type Move = (Peg, Peg);

pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut res: Vec<Move> = Vec::new();

    if num_discs == 1u32 {
        res.push((src, dst));
        return res;
    } else {
    
        let res1 = hanoi(num_discs-1, src, dst, aux);

        for n1 in res1 {
            res.push(n1);
        }

        let res2 = hanoi(num_discs-1, aux, src, dst);

        res.push((src, dst));
        for n2 in res2 {
            res.push(n2);
        }

        return res;
    }
}
