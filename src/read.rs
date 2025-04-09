// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://github.com/bioforensics/mhrs/
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

extern crate serde;

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// Haplotype observation for one read sequence, represented as a sketch at predetermined SNP positions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReadHaplotype {
    alleles: Vec<char>,
}

impl ReadHaplotype {
    /// Allocate space for a haplotype represented by N SNPs.
    pub fn new(size: usize) -> ReadHaplotype {
        let alleles = vec!['N'; size];
        ReadHaplotype { alleles }
    }

    /// Initialize a haplotype observation from a string of SNP alleles.
    pub fn from_string(allele_str: &str) -> ReadHaplotype {
        let alleles = allele_str.chars().collect();
        ReadHaplotype { alleles }
    }

    /// Set the SNP at the given index to the specified allele.
    pub fn set(&mut self, index: usize, allele: char) {
        if index >= self.alleles.len() {
            panic!("index error: {}", index);
        }
        self.alleles[index] = allele;
    }

    /// Indicate whether the read haplotype is complete, i.e., whether any N characters remain.
    pub fn is_partial(&self) -> bool {
        self.alleles.iter().any(|&a| a == 'N')
    }
}

impl fmt::Display for ReadHaplotype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for allele in &self.alleles {
            write!(f, "{}", allele).unwrap();
        }
        write!(f, "")
    }
}

impl Serialize for ReadHaplotype {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ReadHaplotype {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(ReadHaplotype::from_string(&s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ReadHaplotype {
        pub fn get(&self, index: usize) -> Option<char> {
            self.alleles.get(index).copied()
        }

        pub fn len(&self) -> usize {
            self.alleles.len()
        }
    }

    #[test]
    fn test_readhap_basic() {
        let mut hap = ReadHaplotype::new(3);
        hap.set(0, 'C');
        hap.set(1, 'A');
        assert!(hap.is_partial());
        hap.set(2, 'T');
        assert!(!hap.is_partial());
        assert_eq!(hap.to_string(), "CAT");
        assert_eq!(hap.get(2).unwrap(), 'T');
        assert!(hap.get(3).is_none());
    }

    #[test]
    fn test_readhap_from_string() {
        let hap = ReadHaplotype::from_string("GATTACA");
        assert_eq!(hap.len(), 7);
        assert_eq!(hap.get(3).unwrap(), 'T');
        assert_eq!(hap.get(4).unwrap(), 'A');
        assert_eq!(hap.to_string(), "GATTACA");
    }

    #[test]
    #[should_panic(expected = "index error: 42")]
    fn test_readhap_seq_bad_index() {
        let mut hap = ReadHaplotype::from_string("GATTACA");
        hap.set(42, 'T');
    }

    #[test]
    fn test_readhap_serialize() {
        let hap = ReadHaplotype::from_string("GATTACA");
        assert_eq!(
            "\"GATTACA\"",
            serde_json::to_string_pretty(&hap).expect("JSON fail")
        );
    }

    #[test]
    fn test_readhap_equality() {
        assert_eq!(
            ReadHaplotype::from_string("CAT"),
            ReadHaplotype::from_string("CAT")
        );
        assert_ne!(
            ReadHaplotype::from_string("TTT"),
            ReadHaplotype::from_string("GGG")
        );
    }

    #[test]
    fn test_readhap_inequality() {
        assert!(ReadHaplotype::from_string("CAT") <= ReadHaplotype::from_string("CAT"));
        assert!(ReadHaplotype::from_string("CAT") >= ReadHaplotype::from_string("CAT"));
        assert!(ReadHaplotype::from_string("TTT") > ReadHaplotype::from_string("GGG"));
        assert!(ReadHaplotype::from_string("TTT") >= ReadHaplotype::from_string("GGG"));
        assert!(ReadHaplotype::from_string("AAA") < ReadHaplotype::from_string("CCC"));
        assert!(ReadHaplotype::from_string("AAA") <= ReadHaplotype::from_string("CCC"));
    }

    #[test]
    fn test_readhap_sort() {
        let mut observed = vec![
            ReadHaplotype::from_string("CAT"),
            ReadHaplotype::from_string("TAG"),
            ReadHaplotype::from_string("ACT"),
            ReadHaplotype::from_string("TAT"),
            ReadHaplotype::from_string("GAG"),
        ];
        observed.sort();
        let expected = vec![
            ReadHaplotype::from_string("ACT"),
            ReadHaplotype::from_string("CAT"),
            ReadHaplotype::from_string("GAG"),
            ReadHaplotype::from_string("TAG"),
            ReadHaplotype::from_string("TAT"),
        ];
        assert_eq!(observed, expected);
    }
}
