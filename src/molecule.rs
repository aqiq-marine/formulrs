use crate::{atom::*, bond::*};

pub struct Molecule<A: AtomLike, B: BondLike> {
    pub atoms: Vec<A>,
    pub bonds: Vec<(usize, usize, B)>,
}
