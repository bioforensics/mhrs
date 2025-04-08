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
