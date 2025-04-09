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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Threshold manager for a single microhap.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypingThresholds {
    pub dynamic: f64,
    pub analytical: f64,
    pub detection: u16,
}

/// Threshold map for an entire microhap panel.
pub struct ReadCountThreshold<T> {
    default: T,
    by_marker: HashMap<String, T>,
}

impl<T: Clone> ReadCountThreshold<T> {
    pub fn new(default: T) -> ReadCountThreshold<T> {
        ReadCountThreshold {
            default,
            by_marker: HashMap::new(),
        }
    }

    pub fn insert(&mut self, marker_id: &str, threshold: T) {
        self.by_marker.insert(marker_id.to_string(), threshold);
    }

    pub fn get(&self, marker_id: &str) -> T {
        match self.by_marker.get(marker_id) {
            Some(threshold) => threshold.clone(),
            None => self.default.clone(),
        }
    }
}

pub type DetectionThreshold = ReadCountThreshold<u16>;
pub type AnalyticalThreshold = ReadCountThreshold<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_threshold() {
        let mut t = DetectionThreshold::new(42);
        t.insert("mh09USC-9qC", 5);
        t.insert("mh10WL-031", 15);
        assert_eq!(t.get("mh09USC-9qC"), 5);
        assert_eq!(t.get("mh10WL-031"), 15);
        assert_eq!(t.get("mh04SHY-004"), 42);
    }

    #[test]
    fn test_analytical_threshold() {
        let mut t = AnalyticalThreshold::new(0.042);
        t.insert("mh13KK-221.v1", 0.033);
        t.insert("mh19USC-19qB.v2", 0.029);
        assert_eq!(t.get("mh13KK-221.v1"), 0.033);
        assert_eq!(t.get("mh19USC-19qB.v2"), 0.029);
        assert_eq!(t.get("mh09WL-034"), 0.042);
    }
}
