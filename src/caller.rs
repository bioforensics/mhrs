extern crate counter;
extern crate serde;
extern crate serde_json;

use crate::counter::ReadHapCounter;
use crate::observer::HaplotypeObserver;
use crate::read::ReadHaplotype;
use crate::result::{TypingCoverage, TypingResult};
use crate::thresholds::TypingThresholds;
use counter::Counter;

pub struct HaplotypeCaller {
    detection_filter: u16,
    analytical_filter: f64,
    raw_counts: Counter<ReadHaplotype>,
    counts: Counter<ReadHaplotype>,
    coverage: TypingCoverage,
    discarded: usize,
}

impl HaplotypeCaller {
    pub fn from_observer(observer: &HaplotypeObserver) -> HaplotypeCaller {
        let (max, mean, min) = observer.coverage();
        let coverage = TypingCoverage { max, mean, min };
        HaplotypeCaller {
            detection_filter: 0,
            analytical_filter: 0.0,
            raw_counts: observer.count(),
            counts: observer.count(),
            coverage,
            discarded: observer.discarded(),
        }
    }

    pub fn apply_filters(&mut self, detection: u16, analytical: f64) -> TypingResult {
        self.detection_filter = detection;
        self.analytical_filter = analytical;
        let detect = self.apply_detection_threshold();
        let analy = self.apply_analytical_threshold();
        let mut genotype: Vec<ReadHaplotype> = self.counts.keys().cloned().collect();
        genotype.sort();

        let thresholds = TypingThresholds {
            dynamic: analytical,
            analytical: analy,
            detection: detect,
        };
        let counts = ReadHapCounter {
            tally: self.raw_counts.clone(),
        };
        TypingResult {
            genotype,
            coverage: self.coverage.clone(),
            num_discarded: self.discarded,
            thresholds,
            counts,
        }
    }

    fn apply_detection_threshold(&mut self) -> u16 {
        let static_threshold = self.detection_filter;
        self.counts
            .retain(|_, count| *count >= static_threshold as usize);
        static_threshold
    }

    fn apply_analytical_threshold(&mut self) -> f64 {
        let count_total: usize = self.counts.total();
        let dynamic_threshold = (count_total as f64) * self.analytical_filter;
        self.counts
            .retain(|_, count| *count >= dynamic_threshold as usize);
        dynamic_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::AlleleDefinition;

    fn init_caller() -> HaplotypeCaller {
        let def = AlleleDefinition::from_vector(
            "chr22".to_string(),
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
    fn test_typing_basic() {
        let caller = init_caller();
        let readhap1 = ReadHaplotype::from_string("AATAT");
        let readhap2 = ReadHaplotype::from_string("ACGAT");
        assert_eq!(*caller.raw_counts.get(&readhap1).unwrap(), 3);
        assert_eq!(*caller.raw_counts.get(&readhap2).unwrap(), 1);
    }
}
