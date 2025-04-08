extern crate counter;
extern crate rust_htslib;

use crate::definition::AlleleDefinition;
use crate::read::ReadHaplotype;
use counter::Counter;
use rust_htslib::{bam, bam::Read};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct HaplotypeObserver {
    definition: AlleleDefinition,
    index: HashMap<String, ReadHaplotype>,
    depths: Vec<u32>,
}

impl HaplotypeObserver {
    pub fn new(definition: &AlleleDefinition) -> HaplotypeObserver {
        HaplotypeObserver {
            definition: definition.clone(),
            index: HashMap::new(),
            depths: Vec::new(),
        }
    }

    pub fn set(&mut self, read_name: &str, offset: u32, base: char) {
        let num_snps = self.definition.num_snps();
        let readhap = self
            .index
            .entry(read_name.to_string())
            .or_insert(ReadHaplotype::new(num_snps));
        let index = match self.definition.get_index(offset) {
            Some(i) => i,
            None => panic!("invalid offset: {}", offset),
        };
        readhap.set(*index, base);
    }

    pub fn count(&self) -> Counter<ReadHaplotype> {
        let mut counts: Counter<ReadHaplotype> = self.index.values().cloned().collect();
        counts.retain(|readhap, _| !readhap.is_partial());
        counts
    }

    pub fn discarded(&self) -> usize {
        let counts: Counter<ReadHaplotype> = self.index.values().cloned().collect();
        counts
            .iter()
            .filter(|(readhap, _)| readhap.is_partial())
            .map(|(_, count)| count)
            .sum()
    }

    pub fn coverage(&self) -> (u32, f64, u32) {
        let max = match self.depths.iter().max() {
            Some(d) => *d,
            None => 0,
        };
        let min = match self.depths.iter().min() {
            Some(d) => *d,
            None => 0,
        };
        let total: u32 = Iterator::sum(self.depths.iter());
        let mean: f64 = (total as f64) / (self.depths.len() as f64);

        (max, mean, min)
    }

    pub fn is_ads(&self, offset: u32) -> bool {
        self.definition.is_ads(offset)
    }

    pub fn call_from_bam(&mut self, bam_path: &PathBuf, min_base_qual: u8, max_depth: u32) {
        let mut bam = bam::IndexedReader::from_path(bam_path).unwrap();
        let _ = bam.fetch(self.definition.region());
        let mut pileups = bam.pileup();
        pileups.set_max_depth(max_depth);
        for pileup in pileups {
            let pileup = pileup.expect("error reading pileup");
            let refr_pos = pileup.pos();
            if refr_pos >= self.definition.start() && refr_pos <= self.definition.end() {
                self.depths.push(pileup.depth());
            }
            if !self.is_ads(refr_pos) {
                continue;
            }
            for alignment in pileup.alignments() {
                if let Some(qpos) = alignment.qpos() {
                    let record = alignment.record();
                    if Self::skip_record(&record, qpos, min_base_qual) {
                        continue;
                    }
                    let is_gap = alignment.is_del() || alignment.is_refskip();
                    let allele = match is_gap {
                        true => '-',
                        false => record.seq()[qpos] as char,
                    };
                    let read_name = std::str::from_utf8(record.qname()).unwrap();
                    self.set(read_name, refr_pos, allele);
                }
            }
        }
    }

    fn skip_record(record: &bam::record::Record, qpos: usize, min_base_qual: u8) -> bool {
        let ignore = record.is_secondary()
            || record.is_supplementary()
            || record.is_duplicate()
            || record.is_quality_check_failed();
        if ignore {
            return true;
        }
        let base_quality = record.qual()[qpos];
        base_quality < min_base_qual
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl HaplotypeObserver {
        pub fn set_all(&mut self, read_name: &str, alleles: &str) {
            let num_snps = self.definition.num_snps();
            if alleles.len() != num_snps {
                panic!("length mismatch: {} vs {}", alleles.len(), num_snps);
            }
            let readhap = self
                .index
                .entry(read_name.to_string())
                .or_insert(ReadHaplotype::new(num_snps));
            for (index, base) in alleles.chars().enumerate() {
                readhap.set(index, base);
            }
        }
    }

    #[test]
    fn test_observer_basic() {
        let def = AlleleDefinition::from_vector(
            "chr22",
            vec![48665164, 48665175, 48665182, 48665204, 48665216],
        );
        let mut observer = HaplotypeObserver::new(&def);
        observer.set_all("read1", "AATAT");
        observer.set_all("read2", "AATAT");
        observer.set_all("read3", "ACGAT");
        observer.set_all("read4", "AATAT");
        let counts = observer.count();
        assert_eq!(counts.len(), 2);
        let readhap1 = ReadHaplotype::from_string("AATAT");
        let readhap2 = ReadHaplotype::from_string("ACGAT");
        assert_eq!(*counts.get(&readhap1).unwrap(), 3);
        assert_eq!(*counts.get(&readhap2).unwrap(), 1);
    }

    #[test]
    #[should_panic(expected = "invalid offset: 12345")]
    fn test_observer_set_bad_offset() {
        let def = AlleleDefinition::from_vector(
            "chr22",
            vec![48665164, 48665175, 48665182, 48665204, 48665216],
        );
        let mut observer = HaplotypeObserver::new(&def);
        observer.set("read42", 12345, 'C');
    }

    #[test]
    #[should_panic(expected = "length mismatch: 3 vs 5")]
    fn test_observer_set_all_length_mismatch() {
        let def = AlleleDefinition::from_vector(
            "chr22",
            vec![48665164, 48665175, 48665182, 48665204, 48665216],
        );
        let mut observer = HaplotypeObserver::new(&def);
        observer.set_all("read42", "CAT");
    }
}
