// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://maestro.dhs.gov/gitlab-ce/nbfac/mhrs
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

//! mhrs: empirical microhaplotype calling tools written in Rust
//!
//! ```
//! mhrs allele-defn.csv read-aligns.bam --static 20 --dynamic 0.039 > profile.json
//! ```

mod analyzer;
mod caller;
mod cli;
mod counter;
mod definition;
mod observer;
mod panel;
mod parameters;
mod profile;
mod read;
mod result;
mod thresholds;

use analyzer::MicrohapAnalyzer;
use clap::Parser;
use cli::Cli;
use parameters::TypingParameters;

fn main() {
    let args = Cli::parse();
    let mut analyzer = MicrohapAnalyzer::new(&args.sample, &args.csv);
    analyzer.parameters = TypingParameters::new(
        args.detection_threshold,
        args.analytical_threshold,
        args.min_base_quality,
        args.max_depth,
        args.threshold_csv.as_ref(),
    );
    analyzer.process(&args.bam);
    println!("{}", analyzer.final_profile().to_json());
}
