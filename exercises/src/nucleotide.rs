use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct Nucleotide {
    name: String,
    symbol: String,
    complement: String,
}

#[allow(dead_code)]
impl Nucleotide {
    pub fn new(name: &str) -> Nucleotide {
        match name {
            "A" => Nucleotide {
                name: "Adenine".to_string(),
                symbol: "A".to_string(),
                complement: "T".to_string(),
            },
            "C" => Nucleotide {
                name: "Cytosine".to_string(),
                symbol: "C".to_string(),
                complement: "G".to_string(),
            },
            "G" => Nucleotide {
                name: "Guanine".to_string(),
                symbol: "G".to_string(),
                complement: "C".to_string(),
            },
            "T" => Nucleotide {
                name: "Thymine".to_string(),
                symbol: "T".to_string(),
                complement: "A".to_string(),
            },
            "U" => Nucleotide {
                name: "Uracil".to_string(),
                symbol: "U".to_string(),
                complement: "A".to_string(),
            },
            _ => panic!("Invalid nucleotide name"),
        }
    }

    fn get_complement(&self) -> String {
        self.complement.clone()
    }

    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Display for Nucleotide {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Nucleotide {{ name: {}, symbol: {}, complement: {} }}",
            self.name, self.symbol, self.complement
        )
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_nucleotide() {
        let nucleotide = Nucleotide::new("A");
        assert_eq!(nucleotide.get_name(), "Adenine");
        assert_eq!(nucleotide.get_symbol(), "A");
        assert_eq!(nucleotide.get_complement(), "T");
    }

    #[test]
    #[should_panic]
    fn test_nucleotide_panic() {
        Nucleotide::new("I");
    }

    #[test]
    fn test_nucleotide_display() {
        let nucleotide = Nucleotide::new("A");
        assert_eq!(
            format!("{}", nucleotide),
            "Nucleotide { name: Adenine, symbol: A, complement: T }"
        );
    }

    #[test]
    fn test_nucleotide_eq() {
        let nucleotide1 = Nucleotide::new("A");
        let nucleotide2 = Nucleotide::new("A");
        assert_eq!(nucleotide1, nucleotide2);
    }

    #[test]
    fn test_nucleotide_ne() {
        let nucleotide1 = Nucleotide::new("A");
        let nucleotide2 = Nucleotide::new("C");
        assert_ne!(nucleotide1, nucleotide2);
    }

    #[test]
    fn test_nucleotide_ne_different_name() {
        let nucleotide1 = Nucleotide::new("A");
        let nucleotide2 = Nucleotide::new("C");
        assert_ne!(nucleotide1, nucleotide2);
    }

    #[test]
    fn test_nucleotide_complement_adenine() {
        let nucleotide = Nucleotide::new("A");
        assert_eq!(nucleotide.get_complement(), "T");
    }

    #[test]
    fn test_nucleotide_complement_cytosine() {
        let nucleotide = Nucleotide::new("C");
        assert_eq!(nucleotide.get_complement(), "G");
    }

    #[test]
    fn test_nucleotide_complement_guanine() {
        let nucleotide = Nucleotide::new("G");
        assert_eq!(nucleotide.get_complement(), "C");
    }

    #[test]
    fn test_nucleotide_complement_thymine() {
        let nucleotide = Nucleotide::new("T");
        assert_eq!(nucleotide.get_complement(), "A");
    }
}
