// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://github.com/bioforensics/mhrs/
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

use std::collections::HashMap;

/// Description of the allele-defining SNPs that are used to distinguish different haplotypes
/// observed at a microhap locus.
#[derive(Clone, Debug)]
pub struct AlleleDefinition {
    pub chromosome: String,
    offsets: Vec<u32>,
    indices: HashMap<u32, usize>,
}

impl AlleleDefinition {
    /// Initialize a new empty definition.
    pub fn new(chromosome: &str) -> AlleleDefinition {
        AlleleDefinition {
            chromosome: chromosome.to_string(),
            offsets: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Add a SNP to an existing allele definition, indicating its (0-based) position in the genome.
    pub fn add_snp_offset(&mut self, offset: u32) {
        self.offsets.push(offset);
        // Yeah, these next two statements make me cringe too. But this is the opposite of
        // performance-critical code, so I'm opting for code clarity and simplicity over efficiency.
        // --DSS, 2025-04-02
        self.offsets.sort();
        self.indices = self
            .offsets
            .iter()
            .enumerate()
            .map(|(index, &offset)| (offset, index))
            .collect();
    }

    /// Number of SNPs used to define alleles at this microhap locus.
    pub fn num_snps(&self) -> usize {
        self.offsets.len()
    }

    /// Given a genomic coordinate, return the index of the corresponding SNP in this allele definition.
    pub fn get_index(&self, offset: u32) -> Option<&usize> {
        self.indices.get(&offset)
    }

    /// Indicate whether the given (0-based) genomic coordinate points to an allele-defining SNP (ADS).
    pub fn is_ads(&self, offset: u32) -> bool {
        self.indices.contains_key(&offset)
    }

    /// Genomic coordinate of the first SNP in this allele definition.
    pub fn start(&self) -> u32 {
        self.offsets[0] as u32
    }

    /// Genomic coordinate of the last SNP in this allele definition.
    pub fn end(&self) -> u32 {
        self.offsets[self.offsets.len() - 1] as u32
    }

    /// A 3-tuple indicating the absolute genomic position of this microhap allele, in the format (chrom:start-end).
    pub fn region(&self) -> (&str, u32, u32) {
        (&self.chromosome, self.start(), self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl AlleleDefinition {
        pub fn from_vector(chromosome: &str, mut offsets: Vec<u32>) -> AlleleDefinition {
            offsets.sort();
            let indices = offsets
                .iter()
                .enumerate()
                .map(|(index, &offset)| (offset, index))
                .collect();
            AlleleDefinition {
                chromosome: chromosome.to_string(),
                offsets,
                indices,
            }
        }

        pub fn extent(&self) -> usize {
            match self.offsets.len() {
                0 => 0,
                _ => {
                    let start = self.offsets[0];
                    let end = self.offsets[self.offsets.len() - 1];
                    (end - start + 1) as usize
                }
            }
        }

        pub fn get_offsets(&self) -> &Vec<u32> {
            &self.offsets
        }
    }

    #[test]
    fn test_definition_basic() {
        let mut def = AlleleDefinition::new("chr18");
        assert_eq!(def.extent(), 0);
        def.add_snp_offset(53008000);
        def.add_snp_offset(53008025);
        def.add_snp_offset(53008042);
        def.add_snp_offset(53008078);
        assert_eq!(def.extent(), 79);
        assert_eq!(def.num_snps(), 4);
        assert_eq!(def.chromosome, "chr18");
        let observed = def.get_offsets();
        let expected: Vec<u32> = vec![53008000, 53008025, 53008042, 53008078];
        assert_eq!(observed, &expected);
    }

    #[test]
    fn test_definition_construct_from_vector() {
        let def = AlleleDefinition::from_vector("chr13", vec![29218045, 29218056, 29218077]);
        assert_eq!(def.extent(), 33);
        assert_eq!(def.num_snps(), 3);
    }

    #[test]
    fn test_coordinates() {
        let def = AlleleDefinition::from_vector(
            "chr5",
            vec![
                31094962, 31095011, 31095136, 31095187, 31095193, 31095262, 31095306,
            ],
        );
        assert_eq!(def.start(), 31094962);
        assert_eq!(def.end(), 31095306);
        assert_eq!(def.region(), ("chr5", 31094962, 31095306));
        assert!(def.is_ads(31095136));
        assert!(!def.is_ads(31095137));
    }
}
