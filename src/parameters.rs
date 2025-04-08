use crate::thresholds::{AnalyticalThreshold, DetectionThreshold};
use csv::ReaderBuilder;
use std::path::PathBuf;

pub struct TypingParameters {
    pub detection_threshold: DetectionThreshold,
    pub analytical_threshold: AnalyticalThreshold,
    pub min_base_quality: u8,
    pub max_depth: u32,
}

impl TypingParameters {
    pub fn defaults() -> TypingParameters {
        TypingParameters {
            detection_threshold: DetectionThreshold::new(10),
            analytical_threshold: AnalyticalThreshold::new(0.04),
            min_base_quality: 10,
            max_depth: 1e6 as u32,
        }
    }

    pub fn new(
        detection_default: u16,
        analytical_default: f64,
        min_base_quality: u8,
        max_depth: u32,
        thresholds_file: Option<&PathBuf>,
    ) -> TypingParameters {
        let mut params = TypingParameters {
            detection_threshold: DetectionThreshold::new(detection_default),
            analytical_threshold: AnalyticalThreshold::new(analytical_default),
            min_base_quality,
            max_depth,
        };
        match thresholds_file {
            None => (),
            Some(csv_path) => params.parse_thresholds_csv(csv_path),
        };

        params
    }

    fn parse_thresholds_csv(&mut self, csv_path: &PathBuf) {
        let mut reader = ReaderBuilder::new()
            .from_path(csv_path)
            .expect("error parsing CSV file");
        for result in reader.records() {
            let record = result.expect("error parsing CSV record");
            let marker = &record[0];
            let static_th = record[1]
                .parse::<u16>()
                .expect("error parsing detection threshold");
            let dynamic_th = record[2]
                .parse::<f64>()
                .expect("error parsing analytical threshold");
            self.detection_threshold.insert(marker, static_th);
            self.analytical_threshold.insert(marker, dynamic_th);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typing_parameters_defaults() {
        let params = TypingParameters::defaults();
        assert_eq!(params.min_base_quality, 10);
        assert_eq!(params.max_depth, 1000000);
        assert_eq!(params.detection_threshold.get("mh13KK-221.v1"), 10);
        assert_eq!(params.analytical_threshold.get("mh13KK-221.v1"), 0.04);
    }

    #[test]
    fn test_typing_parameters_basic() {
        let params = TypingParameters::new(15, 0.032, 20, 50000, None);
        assert_eq!(params.min_base_quality, 20);
        assert_eq!(params.max_depth, 50000);
        assert_eq!(params.detection_threshold.get("mh13KK-221.v1"), 15);
        assert_eq!(params.analytical_threshold.get("mh13KK-221.v1"), 0.032);
    }

    #[test]
    fn test_typing_parameters_csv() {
        let csv = PathBuf::from("testdata/mwgfour-thresholds.csv");
        let params = TypingParameters::new(12, 0.024, 16, 64000, Some(&csv));
        assert_eq!(params.min_base_quality, 16);
        assert_eq!(params.max_depth, 64000);
        assert_eq!(params.detection_threshold.get("mh03USC-3qC.v2"), 10);
        assert_eq!(params.analytical_threshold.get("mh03USC-3qC.v2"), 0.039);
        assert_eq!(params.detection_threshold.get("mh04WL-052.v1"), 10);
        assert_eq!(params.analytical_threshold.get("mh04WL-052.v1"), 0.031);
        assert_eq!(params.detection_threshold.get("mh06SCUZJ-0528857"), 20);
        assert_eq!(params.analytical_threshold.get("mh06SCUZJ-0528857"), 0.041);
        assert_eq!(params.detection_threshold.get("mh17FHL-005.v3"), 10);
        assert_eq!(params.analytical_threshold.get("mh17FHL-005.v3"), 0.027);
        assert_eq!(params.detection_threshold.get("mh05KK-170.v3"), 12);
        assert_eq!(params.analytical_threshold.get("mh05KK-170.v3"), 0.024);
    }
}
