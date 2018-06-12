/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut ret: Vec<Move> = Vec::new();
    if num_discs <= 1 {
        ret.push((src, dst));
        return ret;
    } else {
        /* Move n-1 from src to aux */
        let mut tmp: Vec<Move> = hanoi(num_discs - 1, src, dst, aux);
        ret.append(&mut tmp);

        /* Move last one from src to dst */
        ret.push((src, dst));

        /* Move those n-1 from aux to dst */
        let mut tmp: Vec<Move> = hanoi(num_discs - 1, aux, src, dst);
        ret.append(&mut tmp);
        return ret;
    }
}


