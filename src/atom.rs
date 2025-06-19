use crate::element::*;
use either::Either;

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
impl<T: ElementLike> From<T> for Atom<T> {
    fn from(value: T) -> Self {
        Self { element: value, charge: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmilesAtom {
    element: FuzzyNuclide,
    charge: i8,
    chiral: Option<Chirality>,
    is_aromatic: bool,
    h_count: u8,
    id: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chirality {
    R, 
    S,
}

impl ChemObject for SmilesAtom {
}

impl AtomLike for SmilesAtom {
    fn element(&self) -> impl ElementLike {
        self.element.clone()
    }
    fn charge(&self) -> i8 {
        self.charge
    }
}

impl SmilesAtom {
    pub fn h_count(self, h_count: u8) -> Self {
        Self {h_count, ..self}
    }
    pub fn id(self, id: u8) -> Self {
        Self {id: Some(id), ..self}
    }
    pub fn chiral(self, chiral: Chirality) -> Self {
        Self {chiral: Some(chiral), ..self}
    }
    pub fn aromatic(self) -> Self {
        Self {is_aromatic: true, ..self}
    }
    pub fn charge(self, charge: i8) -> Self {
        Self {charge, ..self}
    }
    pub fn get_h_count(&self) -> u8 {
        self.h_count
    }
    pub fn  get_id(&self) -> Option<u8> {
        self.id
    }
}


impl From<Element> for SmilesAtom {
    fn from(value: Element) -> Self {
        either::Right(value).into()
    }
}
impl From<Nuclide> for SmilesAtom {
    fn from(value: Nuclide) -> Self {
        either::Left(value).into()
    }
}
impl From<FuzzyNuclide> for SmilesAtom {
    fn from(value: FuzzyNuclide) -> Self {
        Self {
            element: value,
            charge: 0,
            is_aromatic: false,
            chiral: None,
            h_count: 0,
            id: None,
        }
    }
}
