pub trait BondLike {
    fn order(&self) -> f32;
}

pub enum BondType {
    Single,
    Double,
    Triple,
}
