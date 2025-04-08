extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypingThresholds {
    pub dynamic: f64,
    pub analytical: f64,
    pub detection: u16,
}

pub struct ReadCountThreshold<T> {
    pub default: T,
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
