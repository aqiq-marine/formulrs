use crate::element::*;
use crate::atom::*;
use crate::bond::*;
use crate::molecule::*;

fn smiles_to_molecule(smiles: String) -> Result<Molecule<Atom, BondType>, SmilesParseError> {
    Err(SmilesParseError::Unknown)
}

pub enum SmilesParseError {
    Unknown,
}
