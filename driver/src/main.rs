mod cli;
mod toolchain;
mod tracer;
mod utils;

use crate::cli::Cli;

use clap::Parser;
use utrace_common::report;

fn main() {
    let args = Cli::parse();
    if args.init {
        toolchain::build();
        toolchain::install();
        return;
    }

    if args.utrace.is_none() {
        println!("Provide the crate path to trace unsafe.");
        return;
    }

    let filter = args
        .filter
        .map(|f| f.into_iter().map(|s| s.trim().to_string()).collect());
    tracer::run(&args.utrace.unwrap());
    report(filter, args.verbose, args.call_trace);
}
