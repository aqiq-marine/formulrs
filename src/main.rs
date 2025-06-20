mod element;
mod atom;
mod bond;
mod molecule;
mod read;

fn main() {
    let smi = "c1ccccc1";
    let mol = read::smiles_to_molecule(smi).unwrap();
    println!("{:?}", mol.atoms);
}

