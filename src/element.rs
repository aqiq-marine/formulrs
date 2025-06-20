use either::Either;

pub trait ElementLike: std::fmt::Debug + Clone {
    fn atomic_number(&self) -> u8;
    fn get_period(&self) -> u8 {
        let a = self.atomic_number();
        match a {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            87..=118 => 7,
            _ => 0,
        }
    }
    fn get_family(&self) -> u8 {
        let a = self.atomic_number();
        match a {
            1 | 3 | 11 | 19 | 37 | 55 | 87 => 1,
            4 | 12 | 20 | 38 | 56 | 88 => 2,
            21 | 39 |  57..=71 | 89..=103 => 3,
            22 | 40 | 72 | 104 => 4,
            23 | 41 | 73 | 105 => 5,
            24 | 42 | 74 | 106 => 6,
            25 | 43 | 75 | 107 => 7,
            26 | 44 | 76 | 108 => 8,
            27 | 45 | 77 | 109 => 9,
            28 | 46 | 78 | 110 => 10,
            29 | 47 | 79 | 111 => 11,
            30 | 48 | 80 | 112 => 12,
            5 | 13 | 31 | 49 | 81 | 113 => 13,
            6 | 14 | 32 | 50 | 82 | 114 => 14,
            7 | 15 | 33 | 51 | 83 | 115 => 15,
            8 | 16 | 34 | 52 | 84 | 116 => 16,
            9 | 17 | 35 | 53 | 85 | 117 => 17,
            2 | 10 | 18 | 36 | 54 | 86 | 118 => 18,
            _ => 0,
        }
    }
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

impl Nuclide {
    pub fn new(proton_num: u8, neutron_num: u8) -> Self {
        Self { proton_num, neutron_num }
    }
}

// impl From<Element> for Nuclide {
//     fn from(value: Element) -> Self {
//         
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    H,                                                                  He,
    Li, Be,                                          B,  C,  N,  O,  F, Ne,
    Na, Mg,                                         Al, Si,  P,  S, Cl, Ar,
     K, Ca, Sc, Ti,  V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr,  Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te,  I, Xe,
    Cs, Ba,
            La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
                Hf, Ta,  W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra,
            Ac, Th, Pa,  U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
                Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
}

impl ElementLike for Element {
    fn atomic_number(&self) -> u8 {
        *self as u8
    }
}

impl Element {
    pub fn all_elements() -> [Element; 118] {
        use Element::*;
        [
            H,                                                                  He,
            Li, Be,                                          B,  C,  N,  O,  F, Ne,
            Na, Mg,                                         Al, Si,  P,  S, Cl, Ar,
             K, Ca, Sc, Ti,  V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
            Rb, Sr,  Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te,  I, Xe,
            Cs, Ba,
                    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
                        Hf, Ta,  W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
            Fr, Ra,
                    Ac, Th, Pa,  U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
                        Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
        ]
    }
    pub fn from_atomic_number(atomic_number: u8) -> Option<Self> {
        if atomic_number > 0 {
            let i = atomic_number - 1;
            Some(Self::all_elements()[i as usize])
        } else {
            None
        }
    }
    pub fn specify_neutrons(&self, neutron_num: u8) -> Nuclide {
        Nuclide { proton_num: self.atomic_number(), neutron_num }
    }
}
