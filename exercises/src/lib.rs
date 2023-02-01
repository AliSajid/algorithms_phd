use std::collections::HashMap;

pub mod aminoacid;
pub mod chap01;
pub mod codon;
pub mod exercise;
pub mod nucleotide;

pub use aminoacid::AminoAcid;
pub use codon::Codon;
pub use exercise::Exercise;
pub use nucleotide::Nucleotide;

pub fn solutions() -> HashMap<String, Box<dyn Exercise>> {
    let mut solutions: HashMap<String, Box<dyn Exercise>> = HashMap::new();
    solutions.insert("1.1.6".to_string(), Box::new(chap01::e1_1_6::Solution));
    solutions
}
