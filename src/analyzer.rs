use crate::caller::HaplotypeCaller;
use crate::observer::HaplotypeObserver;
use crate::panel::MicrohapPanel;
use crate::profile::MicrohapProfile;
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
                self.parameters.detection_threshold,
                self.parameters.analytical_threshold,
            );
            self.profile.add(mhid, result);
        }
    }

    pub fn final_profile(&self) -> &MicrohapProfile {
        &self.profile
    }
}

pub struct TypingParameters {
    pub detection_threshold: u16,
    pub analytical_threshold: f64,
    pub min_base_quality: u8,
    pub max_depth: u32,
}

impl TypingParameters {
    pub fn defaults() -> TypingParameters {
        TypingParameters {
            detection_threshold: 10,
            analytical_threshold: 0.04,
            min_base_quality: 10,
            max_depth: 1e6 as u32,
        }
    }
}
