use either::Either;

pub trait ElementLike: std::fmt::Debug {
    fn atomic_number(&self) -> u8;
}

pub type FuzzyNuclide = Either<Nuclide, Element>;

impl ElementLike for FuzzyNuclide {
    fn atomic_number(&self) -> u8 {
        match *self {
            Self::Left(n) => n.atomic_number(),
            Self::Right(e) => e.atomic_number(),

        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nuclide {
    pub proton_num: u8,
    pub neutron_num: u8,
}

impl ElementLike for Nuclide {
    fn atomic_number(&self) -> u8 {
        self.proton_num
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    H,
}

impl ElementLike for Element {
    fn atomic_number(&self) -> u8 {
        use Element::*;
        match self {
            H => 1,
        }
    }
}
