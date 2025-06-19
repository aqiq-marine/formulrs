use crate::element::*;
use crate::atom::*;
use crate::bond::*;
use crate::molecule::*;
use purr::graph::Builder;
use purr::feature::{ AtomKind, BondKind, Aliphatic, Aromatic };
use purr::read::read;

fn smiles_to_molecule(smiles: String) -> Result<Molecule<SmilesAtom, SmilesBond>, SmilesParseError> {
    let mut builder = Builder::new();
    read(smiles.as_str(), &mut builder, None)
        .map_err(|_| SmilesParseError::ParseError)?;

    purr_to_formulrs(builder).ok_or(SmilesParseError::Invalid)
}

fn purr_to_formulrs(builder: Builder) -> Option<Molecule<SmilesAtom, SmilesBond>> {
    let mol = builder.build().ok()?;
    let mut bonds = vec![vec![]; mol.len()];
    let mut atoms = vec![];

    for (i, atom) in mol.into_iter().enumerate() {
        let a = purr_atom_to_smiles_atom(atom.kind)?;
        atoms.push(a);
        for b in atom.bonds {
            bonds[i].push((b.tid, purr_bond_to_bond(b.kind)));
        }
    }

    let mol = Molecule::new(atoms, bonds);

    Some(mol)
}

pub enum SmilesParseError {
    ParseError,
    Invalid,
    Unknown,
}

macro_rules! aliphatic_to_smiles_atom {
    ($name:expr, $($elm:ident),*) => {
        match $name {
            $(
                Aliphatic::$elm => SmilesAtom::from(Element::$elm),
            )*
        }
    };
}

macro_rules! aromatic_to_smiles_atom {
    ($name:expr, $($elm:ident),*) => {
        match $name {
            $(
                Aromatic::$elm => SmilesAtom::from(Element::$elm).aromatic(),
            )*
        }
    };
}

macro_rules! bracket_element_to_smiles_atom {
    ($name:expr, $($elm:ident),*) => {
        match $name {
            $(
                purr::feature::Element::$elm => Element::$elm,
            )*
        }
    };
}

macro_rules! bracket_aromatic_to_smiles_atom {
    ($name:expr, $($elm:ident),*) => {
        match $name {
            $(
                purr::feature::BracketAromatic::$elm => Element::$elm,
            )*
        }
    };
}

fn purr_atom_to_smiles_atom(atom: AtomKind) -> Option<SmilesAtom> {
    // TODO: isotope
    match atom {
        AtomKind::Star => None,
        AtomKind::Aliphatic(elm) => Some(aliphatic_to_smiles_atom!(elm, B, C, N, O, S, P, F, Cl, Br, I, At, Ts)),
        AtomKind::Aromatic(elm) => Some(aromatic_to_smiles_atom!(elm, B, C, N, O, P, S)),
        AtomKind::Bracket { isotope, symbol, configuration, hcount, charge, map } => match symbol {
            purr::feature::BracketSymbol::Star => None,
            purr::feature::BracketSymbol::Element(elm) => Some(bracket_element_to_smiles_atom!(elm, H, He, Li, Be, B, C, N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca, Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I, Xe, Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og).into()),
            purr::feature::BracketSymbol::Aromatic(elm) => Some(bracket_aromatic_to_smiles_atom!(elm, B, C, N, O, S, P, Se, As).into()),
        },
    }
}


fn purr_bond_to_bond(bond: BondKind) -> SmilesBond {
    match bond {
        BondKind::Elided => SmilesBond::Single,
        BondKind::Single => SmilesBond::Single,
        BondKind::Double => SmilesBond::Double,
        BondKind::Triple => SmilesBond::Triple,
        BondKind::Quadruple => SmilesBond::Quadruple,
        BondKind::Aromatic => SmilesBond::Aromatic,
        BondKind::Up => SmilesBond::Single,
        BondKind::Down => SmilesBond::Single,
    }
}
