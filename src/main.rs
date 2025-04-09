// -------------------------------------------------------------------------------------------------
// Copyright (c) 2025, DHS.
// This file is part of mhrs: https://github.com/bioforensics/mhrs/
//
// This software was prepared for the Department of Homeland Security (DHS) by the Battelle National
// Biodefense Institute, LLC (BNBI) as part of contract HSHQDC-15-C-00064 to manage and operate the
// National Biodefense Analysis and Countermeasures Center (NBACC), a Federally Funded Research and
// Development Center.
// -------------------------------------------------------------------------------------------------

//! ## mhrs: an empirical microhaplotype calling algorithm written in Rust
//!
//! ### Quick reference for the impatient
//!
//! Provided for convenience if this isn't your first rodeo:
//!
//! `mhrs defn.csv aligns.bam --static=12 --dynamic=0.025 > profile.json`
//!
//! Otherwise, read on for more details.
//!
//! ### User guide
//!
//! This program is designed to genotype specific microhaplotype (*microhap* or *MH*) markers using
//! NGS (e.g. Illumina) reads. It's theoretically possible that the reads could come from a
//! whole-genome shotgun sequencing strategy, but the coverage needed for confident genotype calls
//! will typically require targeting the desired regions with something like multiplex PCR
//! amplification or hybridization capture enrichment. This program does not *discover* new MH loci,
//! it is designed only to *genotype* pre-determined user-specified markers.
//!
//! To demonstrate how mhrs works, consider the microhap `mh16WL-040.v1` shown below. It is
//! comprised of four SNPs spanning 24 bp on chromosome 16.
//!
//! ```text
//!           *    **                *
//! AGGCTTCAGGCGGCTACCCGTGAAATCCCAGGTGCTTACCACAA
//! ```
//!
//! Given those SNP locations and a set of reads aligned to that location, mhrs begins by
//! determining the haplotype observed for each individual read—in other words, the combination of
//! alleles observed at the allele-defining SNPs (ADSs). All other positions are ignored, leaving
//! (in this case) a 4 bp sequence as the "allele".
//!
//! ```text
//! AGGCTTCAGGCGGCTACCCGTGAAATCCCAGGTGCTTACCACAA
//! ..........T....CT..............
//! ....G.....C....CC................G...
//! ..........C....CC................G.......
//! ..........C....CC................G..........
//! ..........T....CT................C..........
//! ..........C....CC................G..........
//! ..........T....CT................C..........
//! ..........T....AT................C..........
//! ..........T....CT.......C........C..........
//! ..........C....CC................G..........
//!    .......T....CT................C..........
//!       ....C....CC................G..........
//!             ...CT................C..........
//! ```
//!
//! In the example above, we see six reads with the `CCCG` allele, four reads with the `TCTC`
//! allele, and one read with the `TATC` allele. The first and last reads are discarded since they
//! do not fully span all four ADSs and therefore are unsuitable for empirical haplotype calling.
//! Two reads appear to have sequencing errors at non-target positions, but this does not impact
//! haplotype calling.
//!
//! Next, mhrs applies thresholds to distinguish false alleles (due e.g. to sequencing error) from
//! true alleles. In the example above, the `TATC` allele is likely a false allele. First, a fixed
//! *detection threshold* is applied to eliminate obvious low-level noise. Next, a dynamic
//! *analytical threshold* is computed as a percentage of the total read counts for alleles not
//! eliminated by the detection treshold. The alleles that remain after filters are applied
//! constitute the genotype call for that microhap.
//!
//! Running the mhrs program requires 1) allele definitions (in CSV format) listing ADS coordinates
//! for a panel of microhap targets and 2) read alignments (in BAM format) against the human genome.
//! It is expected that paired-end reads are merged prior to alignment. The expected format of the
//! allele definition CSV is shown in Appendix A below.
//!
//! The user can configure the filtering behavior of the mhrs program with panel-wide default
//! thresholds, locus-specific thresholds (if validation studies have been performed), or a
//! combination of both. Default thresholds can be configured using command-line arguments (run
//! `mhrs --help` for more information), while locus-specific thresholds can be provided in a CSV
//! file, the expected format of which is shown in Appendix B below.
//!
//! The program generates a *typing result* for each microhap in the panel comprised of the genotype
//! call, per-base read coverage statistics, and raw read counts (excluding partial observations for
//! reads that don't span all ADSs). The typing results for all microhaps in the panel constitute a
//! *profile*, which is reported in JSON format. A profile containing a single typing result, based
//! on the example above, is shown here.
//!
//! ```json
//! {
//!   "sample_id": "Sample001",
//!   "typing_results": {
//!     "mh16WL-040.v1": {
//!       "genotype": [
//!         "CCCG",
//!         "TCTC",
//!       ],
//!       "coverage": {
//!         "max": 11,
//!         "mean": 10.568181818181818,
//!         "min": 9
//!       },
//!       "num_discarded": 1,
//!       "thresholds": {
//!         "dynamic": 0.02,
//!         "analytical": 0.2,
//!         "detection": 2
//!       },
//!       "counts": {
//!         "CCCG": 6,
//!         "TATC": 1,
//!         "TCTC": 4
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ### Appendix A: expected format for allele definitions
//!
//! Each line in the table corresponds to a single ADS. The coordinate is 0-based, that is, its
//! distance from the chromosome's first nucleotide.
//!
//! ```csv
//! Marker,Chrom,OffsetHg38
//! mh01WL-006.v3,chr1,236518813
//! mh01WL-006.v3,chr1,236518863
//! mh01WL-006.v3,chr1,236518878
//! mh01WL-006.v3,chr1,236518884
//! mh01WL-006.v3,chr1,236518947
//! mh01WL-006.v3,chr1,236519007
//! mh01WL-006.v3,chr1,236519025
//! mh02KK-134.v2,chr2,160222899
//! mh02KK-134.v2,chr2,160222923
//! mh02KK-134.v2,chr2,160222938
//! mh02KK-134.v2,chr2,160222944
//! mh02KK-134.v2,chr2,160222986
//! mh02KK-134.v2,chr2,160223002
//! mh04FHL-005.v6,chr4,81722743
//! mh04FHL-005.v6,chr4,81722769
//! mh04FHL-005.v6,chr4,81722790
//! mh04FHL-005.v6,chr4,81722818
//! mh04FHL-005.v6,chr4,81722856
//! ```
//!
//! ### Appendix B: expected format for locus-specific typing thresholds
//!
//! Default thresholds are applied to any microhaps absent from this file by present in the allele
//! definition file.
//!
//! ```csv
//! Marker,Detection,Analytical
//! mh03USC-3qC.v2,10,0.039
//! mh04WL-052.v1,10,0.031
//! mh06SCUZJ-0528857,20,0.041
//! mh17FHL-005.v3,10,0.027
//! ```

mod analyzer;
mod caller;
mod counter;
mod definition;
mod observer;
mod panel;
mod parameters;
mod profile;
mod read;
mod result;
mod thresholds;

#[doc(hidden)]
mod cli;

use analyzer::MicrohapAnalyzer;
use clap::Parser;
use cli::Cli;
use parameters::TypingParameters;

#[doc(hidden)]
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
