// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://github.com/bioforensics/mhrs/
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

extern crate counter;
extern crate serde;
extern crate serde_json;

use crate::counter::ReadHapCounter;
use crate::read::ReadHaplotype;
use crate::thresholds::TypingThresholds;
use serde::{Deserialize, Serialize};

/// Typing result for a single microhap.
#[derive(Serialize, Deserialize)]
pub struct TypingResult {
    pub genotype: Vec<ReadHaplotype>,
    pub coverage: TypingCoverage,
    pub num_discarded: usize,
    pub thresholds: TypingThresholds,
    pub counts: ReadHapCounter,
}

/// Read coverage statistics for a single typing result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypingCoverage {
    pub max: u32,
    pub mean: f64,
    pub min: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller::HaplotypeCaller;
    use crate::definition::AlleleDefinition;
    use crate::observer::HaplotypeObserver;
    use std::fs::File;
    use std::io::Read;

    impl TypingResult {
        pub fn to_json(&self) -> String {
            serde_json::to_string_pretty(self).expect("Failed to serialize TypingResult to JSON")
        }

        pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json)
        }

        pub fn from_file(path: &str) -> TypingResult {
            let mut file = File::open(path).expect("Failed to open file");
            let mut data = String::new();
            file.read_to_string(&mut data).expect("Failed to read file");

            TypingResult::from_json(&data).expect("Failed to parse TypingResult from JSON")
        }
    }

    fn init_caller() -> HaplotypeCaller {
        let def = AlleleDefinition::from_vector(
            "chr22",
            vec![48665164, 48665175, 48665182, 48665204, 48665216],
        );
        let mut observer = HaplotypeObserver::new(&def);
        observer.set_all("read1", "AATAT");
        observer.set_all("read2", "AATAT");
        observer.set_all("read3", "ACGAT");
        observer.set_all("read4", "AATAT");

        HaplotypeCaller::from_observer(&observer)
    }

    #[test]
    fn test_typing_result_basic() {
        let mut caller = init_caller();
        let result = caller.apply_filters(2, 0.02);
        let readhap = ReadHaplotype::from_string("AATAT");
        assert_eq!(result.genotype, vec![readhap]);
    }

    #[test]
    fn test_typing_result_json() {
        let result = TypingResult::from_file("testdata/dummy-result.json");
        assert_eq!(result.coverage.max, 2478);
        assert_eq!(result.counts.tally.len(), 10);
        let json = result.to_json();
        assert!(json.contains("\"num_discarded\": 95,"));
        assert!(json.contains("\"GCTTCT\": 1184,\n    \"GGTTTT\": 1,"));
    }
}
