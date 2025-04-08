use crate::caller::HaplotypeCaller;
use crate::observer::HaplotypeObserver;
use crate::panel::MicrohapPanel;
use crate::parameters::TypingParameters;
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
            let detection = self.parameters.detection_threshold.get(mhid);
            let analytical = self.parameters.analytical_threshold.get(mhid);
            let result = caller.apply_filters(detection, analytical);
            self.profile.add(mhid, result);
        }
    }

    pub fn final_profile(&self) -> &MicrohapProfile {
        &self.profile
    }
}
