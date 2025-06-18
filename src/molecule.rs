use crate::{atom::*, bond::*};

pub struct Molecule<A: AtomLike, B: BondLike> {
    pub atoms: Vec<A>,
    pub bonds: Vec<Vec<(usize, B)>>,
}

impl<A: AtomLike, B: BondLike> Molecule<A, B> {
    pub fn new(atoms: Vec<A>, bonds: Vec<Vec<(usize, B)>>) -> Self {
        Self { atoms, bonds }
    }
}
