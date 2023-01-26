use std::collections::HashMap;

pub mod aminoacid;
pub mod chap01;
pub mod codon;
pub mod nucleotide;

pub fn solutions() -> HashMap<String, fn()> {
    let mut solutions: HashMap<String, fn()> = HashMap::new();
    solutions.insert("1_1_6".to_string(), chap01::e1_1_6::solve);
    solutions
}
