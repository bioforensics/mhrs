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
        let profile = MicrohapProfile::new(sample_id);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read::ReadHaplotype;

    #[test]
    fn test_analyzer() {
        let mut analyzer = MicrohapAnalyzer::new("Item2", &PathBuf::from("testdata/mwgfour.csv"));
        analyzer
            .parameters
            .detection_threshold
            .insert("mh17FHL-005.v3", 3);
        analyzer
            .parameters
            .analytical_threshold
            .insert("mh17FHL-005.v3", 0.0001);
        analyzer.process(&PathBuf::from("testdata/mwgfour-p2.bam"));
        let profile = analyzer.final_profile();

        let result1 = profile.get("mh03USC-3qC.v2").unwrap();
        let expected = vec![
            ReadHaplotype::from_string("CCACTGG"),
            ReadHaplotype::from_string("CTACTGG"),
        ];
        assert_eq!(result1.genotype, expected);

        let result2 = profile.get("mh17FHL-005.v3").unwrap();
        let expected = vec![
            ReadHaplotype::from_string("AGTTTC"),
            ReadHaplotype::from_string("AGTTTT"),
            ReadHaplotype::from_string("GCTTCC"),
            ReadHaplotype::from_string("GCTTCT"),
        ];
        assert_eq!(result2.genotype, expected);
    }
}
