mod cli;
mod toolchain;
mod tracer;
mod utils;

use crate::cli::Cli;
use crate::tracer::Tracer;

use clap::Parser;

fn main() {
    let args = Cli::parse();
    if args.init {
        toolchain::build();
        toolchain::install();
        return;
    }

    let mut tracer = Tracer::new();
    if args.utrace.is_none() {
        println!("Provide the crate path to trace unsafe.");
        return;
    }

    tracer.run(&args.utrace.unwrap());
    tracer.report(&args.filter, args.verbose, args.call_trace);
}
