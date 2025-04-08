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
