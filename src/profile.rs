// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://maestro.dhs.gov/gitlab-ce/nbfac/mhrs
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

extern crate serde;
extern crate serde_json;

use crate::result::TypingResult;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct MicrohapProfile {
    sample_id: String,
    #[serde(rename = "typing_results")]
    results: BTreeMap<String, TypingResult>,
}

impl MicrohapProfile {
    pub fn new(sample_id: &str) -> MicrohapProfile {
        MicrohapProfile {
            sample_id: sample_id.to_string(),
            results: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, mhid: &str, result: TypingResult) {
        self.results.insert(mhid.to_string(), result);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize MicrohapProfile to JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    impl MicrohapProfile {
        pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
            serde_json::from_str(json)
        }

        pub fn from_file(path: &str) -> MicrohapProfile {
            let mut file = File::open(path).expect("Failed to open file");
            let mut data = String::new();
            file.read_to_string(&mut data).expect("Failed to read file");

            MicrohapProfile::from_json(&data).expect("Failed to parse TypingResult from JSON")
        }

        pub fn get(&self, mhid: &str) -> Option<&TypingResult> {
            self.results.get(mhid)
        }
    }

    #[test]
    fn test_profile_basic() {
        let mut profile = MicrohapProfile::new("s1");
        assert_eq!(profile.results.len(), 0);
        let result = TypingResult::from_file("testdata/dummy-result.json");
        profile.add("mh17FHL-005.v3", result);
        assert_eq!(profile.results.len(), 1);
    }

    #[test]
    fn test_profile_json() {
        let profile = MicrohapProfile::from_file("testdata/mwgfour-p2-profile.json");
        assert_eq!(profile.sample_id, "Item2");
        assert_eq!(profile.results.len(), 4);
        let (mhid, result) = profile.results.iter().next().expect("iter fail");
        assert_eq!(mhid, "mh03USC-3qC.v2");
        assert_eq!(result.thresholds.analytical, 92.08);
        let json = profile.to_json();
        assert!(json.contains("mh06SCUZJ-0528857"));
        assert!(json.contains("\"ACCGGGCTC\": 1180,"));
    }
}
