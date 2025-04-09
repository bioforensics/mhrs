// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://maestro.dhs.gov/gitlab-ce/nbfac/mhrs
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

use crate::caller::HaplotypeCaller;
use crate::observer::HaplotypeObserver;
use crate::panel::MicrohapPanel;
use crate::parameters::TypingParameters;
use crate::profile::MicrohapProfile;
use std::path::PathBuf;

/// Data structure for managing empirical microhaplotype calling across multiple loci for a given sample.
pub struct MicrohapAnalyzer {
    panel: MicrohapPanel,
    profile: MicrohapProfile,
    pub parameters: TypingParameters,
}

impl MicrohapAnalyzer {
    /// Initialize with a sample ID and the path to an allele definition file in CSV format.
    ///
    /// ```text
    ///Marker,Chrom,OffsetHg38
    /// mh01WL-006.v3,chr1,236518813
    /// mh01WL-006.v3,chr1,236518863
    /// mh01WL-006.v3,chr1,236518878
    /// mh01WL-006.v3,chr1,236518884
    /// mh01WL-006.v3,chr1,236518947
    /// mh01WL-006.v3,chr1,236519007
    /// mh01WL-006.v3,chr1,236519025
    /// mh02KK-134.v2,chr2,160222899
    /// mh02KK-134.v2,chr2,160222923
    /// mh02KK-134.v2,chr2,160222938
    /// mh02KK-134.v2,chr2,160222944
    /// mh02KK-134.v2,chr2,160222986
    /// mh02KK-134.v2,chr2,160223002
    /// mh04FHL-005.v6,chr4,81722743
    /// mh04FHL-005.v6,chr4,81722769
    /// mh04FHL-005.v6,chr4,81722790
    /// mh04FHL-005.v6,chr4,81722818
    /// mh04FHL-005.v6,chr4,81722856
    /// ```
    pub fn new(sample_id: &str, csv_path: &PathBuf) -> MicrohapAnalyzer {
        let panel = MicrohapPanel::from_csv(csv_path).expect("issue parsing panel CSV");
        let profile = MicrohapProfile::new(sample_id);

        MicrohapAnalyzer {
            panel,
            profile,
            parameters: TypingParameters::defaults(),
        }
    }

    /// Perform empirical microhap calling analysis using the read alignments in the specified BAM
    /// file.
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

    /// Reference to the final microhaplotype profile for this sample.
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
