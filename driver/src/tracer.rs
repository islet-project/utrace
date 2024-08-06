use crate::utils::expand_tilde;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use utrace_common::UnsafeKind;
use utrace_common::{Record, Records};

pub struct Tracer;

impl Tracer {
    pub fn run(&mut self, path: &PathBuf) {
        let target_dir = expand_tilde(&path.as_path());
        let target_dir = fs::canonicalize(&target_dir).expect("Failed to get the absosulte path.");
        env::set_current_dir(&target_dir).expect("Failed to change dir to plugin.");

        let out_dir = utrace_common::config::out_dir();
        let path = Path::new(&out_dir);
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
            fs::create_dir_all(&path).unwrap();
        } else {
            fs::create_dir_all(&path).unwrap();
        }

        Command::new("cargo")
            .arg("clean")
            .status()
            .expect("Failed to clean the package.");

        Command::new("rustup")
            .arg("run")
            .arg("utrace")
            .arg("cargo")
            .arg("build")
            .status()
            .expect("Failed to utrace.");
    }

    pub fn report(&self, filter: &Option<String>, verbose: bool, call_trace: bool) {
        let records = Records::load().expect("Failed to read records.");

        println!(
            "{:<20} {:<10} {:<10} {:<10} {:<10}",
            "Crate", "Functions", "Blocks", "Impls", "Traits"
        );

        for record in &records {
            if let Some(krate) = filter {
                if krate == &record.krate {
                    Self::print_items_count(&record);

                    if verbose {
                        record.print_items_list();
                    }

                    if call_trace {
                        self.call_trace(&records.call_graph, &record.graph);
                    }
                }
            } else {
                Self::print_items_count(&record);

                if verbose {
                    record.print_items_list();
                    //                    record.print_call_graph();
                }
            }
        }
    }

    fn print_items_count(record: &Record) {
        let mut functions = 0;
        let mut blocks = 0;
        let mut impls = 0;
        let mut traits = 0;

        for item in &record.items {
            match item.kind {
                UnsafeKind::Function => functions += 1,
                UnsafeKind::Block => blocks += 1,
                UnsafeKind::Impl => impls += 1,
                UnsafeKind::Trait => traits += 1,
            }
        }

        println!(
            "{:<20} {:<10} {:<10} {:<10} {:<10}",
            record.krate, functions, blocks, impls, traits
        );
    }

    pub fn check_unsafe(&self, item: &str) -> String {
        item.to_string()
        /* XXX.
        if self.unsafe_items.contains(item) {
            format!("{} (unsafe)", item)
        } else {
            item.to_string()
        }*/
    }

    pub fn call_trace(
        &self,
        all: &HashMap<String, Vec<String>>,
        sub: &HashMap<String, Vec<String>>,
    ) {
        for caller in sub.keys() {
            println!("{:indent$}{}", "", self.check_unsafe(caller), indent = 0);
            self.visit_graph(&all, &sub, caller, 1);
            println!("");
        }
    }

    fn visit_graph(
        &self,
        all: &HashMap<String, Vec<String>>,
        sub: &HashMap<String, Vec<String>>,
        node: &str,
        depth: usize,
    ) {
        if let Some(callees) = all.get(node) {
            let mut iter = callees.iter().peekable();
            while let Some(callee) = iter.next() {
                if depth == 1 {
                    println!("├── {}", callee);
                } else {
                    if iter.peek().is_some() {
                        println!(
                            "│ {:indent$}├── {}",
                            "",
                            self.check_unsafe(callee),
                            indent = (depth - 1) * 4
                        );
                    } else {
                        println!(
                            "│ {:indent$}└── {}",
                            "",
                            self.check_unsafe(callee),
                            indent = (depth - 1) * 4
                        );
                    }
                }
                if all.contains_key(callee) {
                    self.visit_graph(all, sub, callee, depth + 1);
                }
            }
        }
    }
}
