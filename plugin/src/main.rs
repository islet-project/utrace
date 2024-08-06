#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

mod parser;

use utrace_common::Records;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{interface::Compiler, Queries};

use crate::parser::Parser;

struct Plugin;

impl Callbacks for Plugin {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().enter(|tcx| {
            let mut parser = Parser::new(tcx);
            parser.run();
            parser.save();
        });

        // Debug
        let records = Records::load().unwrap();
        records.print_items_list();
        records.print_call_graph();

        Compilation::Continue
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    rustc_driver::RunCompiler::new(&args, &mut Plugin)
        .run()
        .unwrap();
}
