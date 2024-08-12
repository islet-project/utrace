pub mod config;

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum UnsafeKind {
    Function,
    Block,
    Trait,
    Impl,
}

#[derive(Serialize, Deserialize, Debug, Eq, Clone)]
pub struct UnsafeItem {
    pub kind: UnsafeKind,
    pub name: String,
}

impl UnsafeItem {
    pub fn new(kind: UnsafeKind, name: String) -> Self {
        Self { kind, name }
    }
}

impl Ord for UnsafeItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for UnsafeItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for UnsafeItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub krate: String,
    pub items: BTreeSet<UnsafeItem>,
    pub graph: BTreeMap<String, Vec<String>>,
}

impl Record {
    pub fn new(krate: String) -> Self {
        Self {
            krate,
            items: BTreeSet::new(),
            graph: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, kind: UnsafeKind, name: String) {
        self.items.insert(UnsafeItem::new(
            kind,
            format!("{}{}", self.krate, name.trim()),
        ));
    }

    pub fn add_edge(&mut self, caller: String, callee: String) {
        let caller = format!("{}{}", self.krate, caller);
        self.graph.entry(caller).or_default().push(callee);
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let path = format!("{}/{}.record", path, self.krate);
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(serde_json::to_writer(writer, self)?)
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    fn print_items_list(&self) {
        println!("## Unsafe Item List ({})", self.krate);

        let items: BTreeSet<_> = self
            .items
            .iter()
            .map(|item| format!("type: {:?}, id: {}", item.kind, item.name))
            .collect();

        for item in items {
            println!("- {}", item);
        }
    }

    fn print_items_count(&self) {
        let mut functions = 0;
        let mut blocks = 0;
        let mut impls = 0;
        let mut traits = 0;

        for item in &self.items {
            match item.kind {
                UnsafeKind::Function => functions += 1,
                UnsafeKind::Block => blocks += 1,
                UnsafeKind::Impl => impls += 1,
                UnsafeKind::Trait => traits += 1,
            }
        }

        println!(
            "{:<20} {:<10} {:<10} {:<10} {:<10}",
            self.krate, functions, blocks, impls, traits
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Records {
    // per crate
    raw_data: Vec<Record>,

    // all deps
    unsafe_list: BTreeSet<String>,
    pub call_graph: BTreeMap<String, Vec<String>>,
}

impl Records {
    pub fn load() -> io::Result<Self> {
        let mut raw_data = Vec::new();
        let out_dir = config::out_dir();
        let out_dir = Path::new(&out_dir);

        for entry in fs::read_dir(out_dir)? {
            let entry = entry?;
            raw_data.push(Record::load(entry.path().to_str().unwrap())?);
        }

        let unsafe_list = raw_data
            .iter()
            .flat_map(|record| record.items.iter().map(|item| item.name.clone()))
            .collect();

        let call_graph = raw_data
            .iter()
            .flat_map(|record| record.graph.iter())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        Ok(Self {
            raw_data,
            unsafe_list,
            call_graph,
        })
    }

    fn check_unsafe(&self, item: &str) -> String {
        if self.unsafe_list.contains(item) {
            format!("{} (unsafe)", item)
        } else {
            item.to_string()
        }
    }

    pub fn print_call_trace(&self, krate: &str) {
        println!("## Unsafe Call Trace");
        let all_deps = &self.call_graph;
        let target = self
            .raw_data
            .iter()
            .find(|record| record.krate == krate)
            .map(|record| &record.graph);
        let target = target.unwrap_or(all_deps);

        for caller in target.keys() {
            println!("{:indent$}- {}", "", self.check_unsafe(caller), indent = 0);
            self.visit_callee(all_deps, caller, 1);
        }
    }

    fn visit_callee(&self, graph: &BTreeMap<String, Vec<String>>, caller: &str, depth: usize) {
        if let Some(callees) = graph.get(caller) {
            let iter = callees.iter().peekable();
            for callee in iter {
                if !callee.is_empty() {
                    println!(
                        "{:indent$}- {}",
                        "",
                        self.check_unsafe(callee),
                        indent = depth * 4
                    );

                    if graph.contains_key(callee) {
                        let caller = callee;
                        self.visit_callee(graph, caller, depth + 1);
                    }
                }
            }
        }
    }

    pub fn print_unsafe_list(&self, filter: Option<Vec<String>>) {
        for record in &self.raw_data {
            if let Some(ref krates) = filter {
                for krate in krates {
                    if *krate == record.krate {
                        record.print_items_list();
                    }
                }
            } else {
                record.print_items_list();
            }
        }
    }

    pub fn summary(&self, filter: Option<Vec<String>>) {
        println!("## Summary");
        println!(
            "{:<20} {:<10} {:<10} {:<10} {:<10}",
            "Crate", "Functions", "Blocks", "Impls", "Traits"
        );

        for record in &self.raw_data {
            if let Some(ref krates) = filter {
                for krate in krates {
                    if *krate == record.krate {
                        record.print_items_count();
                    }
                }
            } else {
                record.print_items_count();
            }
        }
    }
}

impl<'a> IntoIterator for &'a Records {
    type Item = <&'a Vec<Record> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Record> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.raw_data.iter()
    }
}

pub fn report(filter: Option<Vec<String>>, verbose: bool, call_trace: bool) {
    let records = Records::load().expect("Failed to read records.");

    records.summary(filter.clone());

    if verbose {
        println!();
        records.print_unsafe_list(filter.clone());
    }

    if call_trace {
        println!();
        for record in &records {
            if let Some(ref krates) = filter {
                for krate in krates {
                    if *krate == record.krate {
                        records.print_call_trace(krate);
                    }
                }
            }
        }
    }
}
