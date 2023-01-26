use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
struct Codon {
    codon: String,
    codon_dna: String,
    special: bool,
    amino_acid: Option<String>,
}

impl Codon {
    pub fn new(codon: &str) -> Self {
        match codon {
            "AAA" => Self {
                codon: "AAA".to_string(),
                codon_dna: "AAA".to_string(),
                special: false,
                amino_acid: Some("Lysine".to_string()),
            },
            "AAC" => Self {
                codon: "AAC".to_string(),
                codon_dna: "AAC".to_string(),
                special: false,
                amino_acid: Some("Asparagine".to_string()),
            },
            "AAG" => Self {
                codon: "AAG".to_string(),
                codon_dna: "AAG".to_string(),
                special: false,
                amino_acid: Some("Lysine".to_string()),
            },
            "AAU" => Self {
                codon: "AAU".to_string(),
                codon_dna: "AAT".to_string(),
                special: false,
                amino_acid: Some("Asparagine".to_string()),
            },
            "ACA" => Self {
                codon: "ACA".to_string(),
                codon_dna: "ACA".to_string(),
                special: false,
                amino_acid: Some("Threonine".to_string()),
            },
            "ACC" => Self {
                codon: "ACC".to_string(),
                codon_dna: "ACC".to_string(),
                special: false,
                amino_acid: Some("Threonine".to_string()),
            },
            "ACG" => Self {
                codon: "ACG".to_string(),
                codon_dna: "ACG".to_string(),
                special: false,
                amino_acid: Some("Threonine".to_string()),
            },
            "ACU" => Self {
                codon: "ACU".to_string(),
                codon_dna: "ACT".to_string(),
                special: false,
                amino_acid: Some("Threonine".to_string()),
            },
            "AGA" => Self {
                codon: "AGA".to_string(),
                codon_dna: "AGA".to_string(),
                special: false,
                amino_acid: Some("Arginine".to_string()),
            },
            "AGC" => Self {
                codon: "AGC".to_string(),
                codon_dna: "AGC".to_string(),
                special: false,
                amino_acid: Some("Serine".to_string()),
            },
            "AGG" => Self {
                codon: "AGG".to_string(),
                codon_dna: "AGG".to_string(),
                special: false,
                amino_acid: Some("Arginine".to_string()),
            },
            "AGU" => Self {
                codon: "AGU".to_string(),
                codon_dna: "AGT".to_string(),
                special: false,
                amino_acid: Some("Serine".to_string()),
            },
            "AUA" => Self {
                codon: "AUA".to_string(),
                codon_dna: "ATA".to_string(),
                special: false,
                amino_acid: Some("Isoleucine".to_string()),
            },
            "AUC" => Self {
                codon: "AUC".to_string(),
                codon_dna: "ATC".to_string(),
                special: false,
                amino_acid: Some("Isoleucine".to_string()),
            },
            "AUG" => Self {
                codon: "AUG".to_string(),
                codon_dna: "ATG".to_string(),
                special: true,
                amino_acid: Some("Methionine".to_string()),
            },
            "AUU" => Self {
                codon: "AUU".to_string(),
                codon_dna: "ATT".to_string(),
                special: false,
                amino_acid: Some("Isoleucine".to_string()),
            },
            "CAA" => Self {
                codon: "CAA".to_string(),
                codon_dna: "CAA".to_string(),
                special: false,
                amino_acid: Some("Glutamine".to_string()),
            },
            "CAC" => Self {
                codon: "CAC".to_string(),
                codon_dna: "CAC".to_string(),
                special: false,
                amino_acid: Some("Histidine".to_string()),
            },
            "CAG" => Self {
                codon: "CAG".to_string(),
                codon_dna: "CAG".to_string(),
                special: false,
                amino_acid: Some("Glutamine".to_string()),
            },
            "CAU" => Self {
                codon: "CAU".to_string(),
                codon_dna: "CAT".to_string(),
                special: false,
                amino_acid: Some("Histidine".to_string()),
            },
            "CCA" => Self {
                codon: "CCA".to_string(),
                codon_dna: "CCA".to_string(),
                special: false,
                amino_acid: Some("Proline".to_string()),
            },
            "CCC" => Self {
                codon: "CCC".to_string(),
                codon_dna: "CCC".to_string(),
                special: false,
                amino_acid: Some("Proline".to_string()),
            },
            "CCG" => Self {
                codon: "CCG".to_string(),
                codon_dna: "CCG".to_string(),
                special: false,
                amino_acid: Some("Proline".to_string()),
            },
            "CCU" => Self {
                codon: "CCU".to_string(),
                codon_dna: "CCT".to_string(),
                special: false,
                amino_acid: Some("Proline".to_string()),
            },
            "CGA" => Self {
                codon: "CGA".to_string(),
                codon_dna: "CGA".to_string(),
                special: false,
                amino_acid: Some("Arginine".to_string()),
            },
            "CGC" => Self {
                codon: "CGC".to_string(),
                codon_dna: "CGC".to_string(),
                special: false,
                amino_acid: Some("Arginine".to_string()),
            },
            "CGG" => Self {
                codon: "CGG".to_string(),
                codon_dna: "CGG".to_string(),
                special: false,
                amino_acid: Some("Arginine".to_string()),
            },
            "CGU" => Self {
                codon: "CGU".to_string(),
                codon_dna: "CGT".to_string(),
                special: false,
                amino_acid: Some("Arginine".to_string()),
            },
            "CUA" => Self {
                codon: "CUA".to_string(),
                codon_dna: "CUA".to_string(),
                special: false,
                amino_acid: Some("Leucine".to_string()),
            },
            "CUC" => Self {
                codon: "CUC".to_string(),
                codon_dna: "CUC".to_string(),
                special: false,
                amino_acid: Some("Leucine".to_string()),
            },
            "CUG" => Self {
                codon: "CUG".to_string(),
                codon_dna: "CUG".to_string(),
                special: true,
                amino_acid: Some("Leucine".to_string()),
            },
            "CUU" => Self {
                codon: "CUU".to_string(),
                codon_dna: "CUT".to_string(),
                special: false,
                amino_acid: Some("Leucine".to_string()),
            },
            "GAA" => Self {
                codon: "GAA".to_string(),
                codon_dna: "GAA".to_string(),
                special: false,
                amino_acid: Some("Glutamic Acid".to_string()),
            },
            "GAC" => Self {
                codon: "GAC".to_string(),
                codon_dna: "GAC".to_string(),
                special: false,
                amino_acid: Some("Aspartic Acid".to_string()),
            },
            "GAG" => Self {
                codon: "GAG".to_string(),
                codon_dna: "GAG".to_string(),
                special: false,
                amino_acid: Some("Glutamic Acid".to_string()),
            },
            "GAU" => Self {
                codon: "GAU".to_string(),
                codon_dna: "GAT".to_string(),
                special: false,
                amino_acid: Some("Aspartic Acid".to_string()),
            },
            "GCA" => Self {
                codon: "GCA".to_string(),
                codon_dna: "GCA".to_string(),
                special: false,
                amino_acid: Some("Alanine".to_string()),
            },
            "GCC" => Self {
                codon: "GCC".to_string(),
                codon_dna: "GCC".to_string(),
                special: false,
                amino_acid: Some("Alanine".to_string()),
            },
            "GCG" => Self {
                codon: "GCG".to_string(),
                codon_dna: "GCG".to_string(),
                special: false,
                amino_acid: Some("Alanine".to_string()),
            },
            "GCU" => Self {
                codon: "GCU".to_string(),
                codon_dna: "GCT".to_string(),
                special: false,
                amino_acid: Some("Alanine".to_string()),
            },
            "GGA" => Self {
                codon: "GGA".to_string(),
                codon_dna: "GGA".to_string(),
                special: false,
                amino_acid: Some("Glycine".to_string()),
            },
            "GGC" => Self {
                codon: "GGC".to_string(),
                codon_dna: "GGC".to_string(),
                special: false,
                amino_acid: Some("Glycine".to_string()),
            },
            "GGG" => Self {
                codon: "GGG".to_string(),
                codon_dna: "GGG".to_string(),
                special: false,
                amino_acid: Some("Glycine".to_string()),
            },
            "GGU" => Self {
                codon: "GGU".to_string(),
                codon_dna: "GGT".to_string(),
                special: false,
                amino_acid: Some("Glycine".to_string()),
            },
            "GUA" => Self {
                codon: "GUA".to_string(),
                codon_dna: "GUA".to_string(),
                special: false,
                amino_acid: Some("Valine".to_string()),
            },
            "GUC" => Self {
                codon: "GUC".to_string(),
                codon_dna: "GUC".to_string(),
                special: false,
                amino_acid: Some("Valine".to_string()),
            },
            "GUG" => Self {
                codon: "GUG".to_string(),
                codon_dna: "GUG".to_string(),
                special: true,
                amino_acid: Some("Valine".to_string()),
            },
            "GUU" => Self {
                codon: "GUU".to_string(),
                codon_dna: "GUT".to_string(),
                special: false,
                amino_acid: Some("Valine".to_string()),
            },
            "UAA" => Self {
                codon: "UAA".to_string(),
                codon_dna: "TAA".to_string(),
                special: true,
                amino_acid: None,
            },
            "UAC" => Self {
                codon: "UAC".to_string(),
                codon_dna: "TAC".to_string(),
                special: false,
                amino_acid: Some("Tyrosine".to_string()),
            },
            "UAG" => Self {
                codon: "UAG".to_string(),
                codon_dna: "TAG".to_string(),
                special: true,
                amino_acid: None,
            },
            "UAU" => Self {
                codon: "UAU".to_string(),
                codon_dna: "TAT".to_string(),
                special: false,
                amino_acid: Some("Tyrosine".to_string()),
            },
            "UCA" => Self {
                codon: "UCA".to_string(),
                codon_dna: "TCA".to_string(),
                special: false,
                amino_acid: Some("Serine".to_string()),
            },
            "UCC" => Self {
                codon: "UCC".to_string(),
                codon_dna: "TCC".to_string(),
                special: false,
                amino_acid: Some("Serine".to_string()),
            },
            "UCG" => Self {
                codon: "UCG".to_string(),
                codon_dna: "TCG".to_string(),
                special: false,
                amino_acid: Some("Serine".to_string()),
            },
            "UCU" => Self {
                codon: "UCU".to_string(),
                codon_dna: "TCT".to_string(),
                special: false,
                amino_acid: Some("Serine".to_string()),
            },
            "UGA" => Self {
                codon: "UGA".to_string(),
                codon_dna: "TGA".to_string(),
                special: true,
                amino_acid: None,
            },
            "UGC" => Self {
                codon: "UGC".to_string(),
                codon_dna: "TGC".to_string(),
                special: false,
                amino_acid: Some("Cysteine".to_string()),
            },
            "UGG" => Self {
                codon: "UGG".to_string(),
                codon_dna: "TGG".to_string(),
                special: false,
                amino_acid: Some("Tryptophan".to_string()),
            },
            "UGU" => Self {
                codon: "UGU".to_string(),
                codon_dna: "TGT".to_string(),
                special: false,
                amino_acid: Some("Cysteine".to_string()),
            },
            "UUA" => Self {
                codon: "UUA".to_string(),
                codon_dna: "TUA".to_string(),
                special: true,
                amino_acid: Some("Leucine".to_string()),
            },
            "UUC" => Self {
                codon: "UUC".to_string(),
                codon_dna: "TUC".to_string(),
                special: false,
                amino_acid: Some("Phenylalanine".to_string()),
            },
            "UUG" => Self {
                codon: "UUG".to_string(),
                codon_dna: "TUG".to_string(),
                special: true,
                amino_acid: Some("Leucine".to_string()),
            },
            "UUU" => Self {
                codon: "UUU".to_string(),
                codon_dna: "TUT".to_string(),
                special: false,
                amino_acid: Some("Phenylalanine".to_string()),
            },
            _ => Self {
                codon: "???".to_string(),
                codon_dna: "???".to_string(),
                special: false,
                amino_acid: None,
            },
        }
    }
}

impl Display for Codon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codon() {
        let codon = Codon::new("AAA");
        assert_eq!(codon.codon, "AAA");
        assert_eq!(codon.codon_dna, "AAA");
        assert_eq!(codon.special, false);
        assert_eq!(codon.amino_acid, Some("Lysine".to_string()));
    }

    #[test]
    fn test_codon_unknown() {
        let codon = Codon::new("???");
        assert_eq!(codon.codon, "???");
        assert_eq!(codon.codon_dna, "???");
        assert_eq!(codon.special, false);
        assert_eq!(codon.amino_acid, None);
    }

    #[test]
    fn test_codon_display() {
        let codon = Codon::new("AAA");
        assert_eq!(codon.to_string(), "AAA");
    }

    #[test]
    fn test_codon_display_unknown() {
        let codon = Codon::new("???");
        assert_eq!(codon.to_string(), "???");
    }

    #[test]
    fn test_codon_special() {
        let codon = Codon::new("UAA");
        assert_eq!(codon.codon, "UAA");
        assert_eq!(codon.codon_dna, "TAA");
        assert_eq!(codon.special, true);
        assert_eq!(codon.amino_acid, None);
    }
}
