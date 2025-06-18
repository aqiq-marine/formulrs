pub trait BondLike {
    fn order(&self) -> f32;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BondType {
    Single,
    Double,
    Triple,
}

impl BondLike for BondType {
    fn order(&self) -> f32 {
        use BondType::*;
        match self {
            Single => 1.0,
            Double => 2.0,
            Triple => 3.0,
        }
    }
}
