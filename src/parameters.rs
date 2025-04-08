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
        detection_global: u16,
        analytical_global: f64,
        min_base_quality: u8,
        max_depth: u32,
        thresholds_file: Option<&PathBuf>,
    ) -> TypingParameters {
        let mut params = TypingParameters {
            detection_threshold: DetectionThreshold::new(detection_global),
            analytical_threshold: AnalyticalThreshold::new(analytical_global),
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
