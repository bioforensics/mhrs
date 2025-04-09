// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://github.com/bioforensics/mhrs/
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Empirical microhaplotype caller", long_about = None)]
pub struct Cli {
    #[arg(help = "Microhap allele definitions in CSV format")]
    pub csv: PathBuf,

    #[arg(help = "Read alignments in BAM format")]
    pub bam: PathBuf,

    #[arg(
        short = 'n',
        long = "name",
        value_name = "SN",
        default_value = "Sample",
        help = "Sample name"
    )]
    pub sample: String,

    #[arg(
        short = 's',
        long = "static",
        value_name = "DT",
        default_value = "10",
        help = "Static detection threshold"
    )]
    pub detection_threshold: u16,

    #[arg(
        short = 'd',
        long = "dynamic",
        value_name = "AT",
        default_value = "0.04",
        help = "Dynamic analytical threshold, as a percentage of total read count after filtering reads that fail the detection threshold"
    )]
    pub analytical_threshold: f64,

    #[arg(
        short = 't',
        long = "threshold-csv",
        value_name = "TC",
        help = "CSV file with marker-specific thresholds; column 1 = marker name, column 2 = detection, column 3 = analytical"
    )]
    pub threshold_csv: Option<PathBuf>,

    #[arg(
        short = 'b',
        long = "base-qual",
        value_name = "BQ",
        default_value = "10",
        help = "Minimum base quality for read haplotype calling"
    )]
    pub min_base_quality: u8,

    #[arg(
        short = 'x',
        long = "max-depth",
        value_name = "MD",
        default_value = "1000000",
        help = "Maximum per-base read depth"
    )]
    pub max_depth: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_defaults() {
        let arglist = vec!["mhrs", "testdata/mwgfour.csv", "testdata/mwgfour-p1p3.bam"];
        let args = Cli::parse_from(arglist);
        assert_eq!(args.csv, PathBuf::from("testdata/mwgfour.csv"));
        assert_eq!(args.bam, PathBuf::from("testdata/mwgfour-p1p3.bam"));
        assert_eq!(args.detection_threshold, 10);
        assert_eq!(args.analytical_threshold, 0.04);
        assert!(args.threshold_csv.is_none());
    }
}
