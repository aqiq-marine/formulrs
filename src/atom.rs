use crate::element::*;

pub trait ChemObject {

}

pub trait AtomLike: ChemObject {
    fn element(&self) -> impl ElementLike;
    fn charge(&self) -> i8;
}

pub struct Atom<T: ElementLike = FuzzyNuclide> {
    pub element: T,
    pub charge: i8,
}

