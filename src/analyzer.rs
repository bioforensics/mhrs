use crate::caller::HaplotypeCaller;
use crate::observer::HaplotypeObserver;
use crate::panel::MicrohapPanel;
use crate::profile::MicrohapProfile;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct MicrohapAnalyzer {
    panel: MicrohapPanel,
    profile: MicrohapProfile,
    pub parameters: TypingParameters,
}

impl MicrohapAnalyzer {
    pub fn new(sample_id: &str, csv_path: &PathBuf) -> MicrohapAnalyzer {
        let panel = MicrohapPanel::from_csv(csv_path).expect("issue parsing panel CSV");
        let profile = MicrohapProfile::new(sample_id.to_string());

        MicrohapAnalyzer {
            panel,
            profile,
            parameters: TypingParameters::defaults(),
        }
    }

    pub fn process(&mut self, bam_path: &PathBuf) {
        for (mhid, definition) in self.panel.iter() {
            let mut observer = HaplotypeObserver::new(definition);
            observer.call_from_bam(
                bam_path,
                self.parameters.min_base_quality,
                self.parameters.max_depth,
            );
            let mut caller = HaplotypeCaller::from_observer(&observer);
            let result = caller.apply_filters(
                self.parameters.detection_threshold.global,
                self.parameters.analytical_threshold.global,
            );
            self.profile.add(mhid, result);
        }
    }

    pub fn final_profile(&self) -> &MicrohapProfile {
        &self.profile
    }
}

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
            self.detection_threshold
                .by_marker
                .insert(marker.to_string(), static_th);
            self.analytical_threshold
                .by_marker
                .insert(marker.to_string(), dynamic_th);
        }
    }
}

pub struct DetectionThreshold {
    pub global: u16,
    pub by_marker: HashMap<String, u16>,
}

impl DetectionThreshold {
    pub fn new(global: u16) -> DetectionThreshold {
        DetectionThreshold {
            global,
            by_marker: HashMap::new(),
        }
    }
}

pub struct AnalyticalThreshold {
    pub global: f64,
    pub by_marker: HashMap<String, f64>,
}

impl AnalyticalThreshold {
    pub fn new(global: f64) -> AnalyticalThreshold {
        AnalyticalThreshold {
            global,
            by_marker: HashMap::new(),
        }
    }
}
