mod analyzer;
mod caller;
mod cli;
mod counter;
mod definition;
mod observer;
mod panel;
mod profile;
mod read;
mod result;

use analyzer::{MicrohapAnalyzer, TypingParameters};
use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    let mut analyzer = MicrohapAnalyzer::new(&args.sample, &args.csv);
    analyzer.parameters = TypingParameters {
        detection_threshold: args.detection_threshold,
        analytical_threshold: args.analytical_threshold,
        min_base_quality: args.min_base_quality,
        max_depth: 1e6 as u32,
    };
    analyzer.process(&args.bam);
    println!("{}", analyzer.final_profile().to_json());
}
