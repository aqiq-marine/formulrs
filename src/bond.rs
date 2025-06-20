pub trait BondLike {
    fn order(&self) -> f32;
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmilesBond {
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
}

impl BondLike for SmilesBond {
    fn order(&self) -> f32 {
        use SmilesBond::*;
        match self {
            Single => 1.0,
            Double => 2.0,
            Triple => 3.0,
            Quadruple => 4.0,
            Aromatic => 1.5,
        }
    }
}
