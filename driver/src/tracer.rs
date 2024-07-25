use crate::utils::expand_tilde;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use utrace_common::config;
use utrace_common::{Record, Records};
use utrace_common::{UnsafeItem, UnsafeKind};

pub struct Tracer {
    records: Records,
    unsafe_items: HashSet<String>,
    call_graph: HashMap<String, Vec<String>>,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            records: Records::new(),
            unsafe_items: HashSet::new(),
            call_graph: HashMap::new(),
        }
    }

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

        self.load().expect("Failed to read records.");

        self.unsafe_items = self.records.collect_items();
        self.call_graph = self.records.collect_graph();
    }

    fn load(&mut self) -> io::Result<()> {
        let out_dir = config::out_dir();
        let out_dir = Path::new(&out_dir);

        for entry in fs::read_dir(&out_dir)? {
            let entry = entry?;
            self.records
                .add(Record::load(entry.path().to_str().unwrap())?);
        }

        Ok(())
    }

    pub fn report(&self, filter: &Option<String>, verbose: bool, call_trace: bool) {
        println!(
            "{:<20} {:<10} {:<10} {:<10} {:<10}",
            "Crate", "Functions", "Blocks", "Impls", "Traits"
        );

        for record in &self.records {
            if let Some(krate) = filter {
                if krate == &record.krate {
                    Self::print_items_count(&record);

                    if verbose {
                        Self::print_items_list(record.items.clone());
                    }

                    if call_trace {
                        self.call_trace(&self.call_graph, &record.graph);
                    }
                }
            } else {
                Self::print_items_count(&record);

                if verbose {
                    Self::print_items_list(record.items.clone());
                    Self::print_call_graph(&record.graph);
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

    fn print_items_list(mut items: Vec<UnsafeItem>) {
        items.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));
        for item in items {
            println!("\tVERBOSE: {:?} - {}", item.kind, item.name);
        }
    }

    fn print_call_graph(graph: &HashMap<String, Vec<String>>) {
        for (caller, callees) in graph {
            println!("{} calls:", caller);
            for callee in callees {
                println!("  - {}", callee);
            }
        }
    }

    pub fn check_unsafe(&self, item: &str) -> String {
        if self.unsafe_items.contains(item) {
            format!("{} (unsafe)", item)
        } else {
            item.to_string()
        }
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
