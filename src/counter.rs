// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://maestro.dhs.gov/gitlab-ce/nbfac/mhrs
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

extern crate counter;
extern crate serde;

use crate::read::ReadHaplotype;
use counter::Counter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

/// Convenience data structure for serializing and deserializing haplotype read counts to and from
/// JSON.
#[derive(Debug, Clone)]
pub struct ReadHapCounter {
    pub tally: Counter<ReadHaplotype>,
}

impl Serialize for ReadHapCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map: BTreeMap<String, usize> = self
            .tally
            .iter()
            .map(|(haplotype, &count)| (haplotype.to_string(), count))
            .collect();
        map.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReadHapCounter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: BTreeMap<String, usize> = BTreeMap::deserialize(deserializer)?;
        let mut tally = Counter::new();
        for (haplotype_str, count) in map {
            let haplotype = ReadHaplotype::from_string(&haplotype_str);
            tally.insert(haplotype, count);
        }

        Ok(ReadHapCounter { tally })
    }
}
