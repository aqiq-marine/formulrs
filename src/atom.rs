use crate::element::*;

pub trait ChemObject {

}

pub trait AtomLike: ChemObject {
    fn element(&self) -> impl ElementLike;
    fn charge(&self) -> i8;
}

pub struct Atom<T: ElementLike = FuzzyNuclide> {
    element: T,
    charge: i8,
}

impl<T: ElementLike> ChemObject for Atom<T> {
}

impl<T: ElementLike> AtomLike for Atom<T> {
    fn element(&self) -> impl ElementLike {
        self.element.clone()
    }
    fn charge(&self) -> i8 {
        self.charge
    }
}
