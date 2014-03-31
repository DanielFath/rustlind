#![crate_id = "rustlind"]
#![feature(globs)]

pub use dna::count_nucleos;

pub mod dna;


pub enum Nucleotide {
    Adenine,
    Cytosine ,
    Guanine,
    Thymine
}

fn main() {

}