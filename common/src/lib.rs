pub mod config;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;
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

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub struct UnsafeItem {
    pub kind: UnsafeKind,
    pub name: String,
}

impl UnsafeItem {
    pub fn new(kind: UnsafeKind, name: String) -> Self {
        Self { kind, name }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub krate: String,
    pub items: Vec<UnsafeItem>,
    pub graph: HashMap<String, Vec<String>>,
}

impl Record {
    pub fn new(krate: String) -> Self {
        Self {
            krate,
            items: Vec::new(),
            graph: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, kind: UnsafeKind, name: String) {
        self.items
            .push(UnsafeItem::new(kind, format!("{}{}", self.krate, name)));
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

    pub fn print_items_list(&self) {
        println!("Unsafe Item List");
        let mut items = self.items.clone();

        items.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));
        for item in items {
            println!("\t{:?} - {}", item.kind, item.name);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Records {
    raw_data: Vec<Record>,
    unsafe_list: BTreeSet<String>,
    pub call_graph: HashMap<String, Vec<String>>,
}

impl Records {
    pub fn load() -> io::Result<Self> {
        let mut raw_data = Vec::new();
        let out_dir = config::out_dir();
        let out_dir = Path::new(&out_dir);

        for entry in fs::read_dir(&out_dir)? {
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

    pub fn print_items_list(&self) {
        println!("Unsafe Item List");
        for per_krate in &self.raw_data {
            let mut items = per_krate.items.clone();

            items.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));
            for item in items {
                println!("\t{:?} - {}", item.kind, item.name);
            }
        }
    }

    pub fn print_call_graph(&self) {
        println!("Unsafe Call Graph");
        let mut sorted_keys: Vec<_> = self.call_graph.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            println!("\t{}", self.check_unsafe(key));
            if let Some(values) = self.call_graph.get(key) {
                for value in values {
                    if !value.is_empty() {
                        println!("\t└─ {}", self.check_unsafe(value));
                    }
                }
            }
        }
    }

    pub fn check_unsafe(&self, item: &str) -> String {
        if self.unsafe_list.contains(item) {
            format!("{} (unsafe)", item)
        } else {
            item.to_string()
        }
    }

    /*
    pub fn save(&self, path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(serde_json::to_writer(writer, self)?)
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }*/

    /*
    pub fn collect_items(&self) -> HashSet<String> {
        self.0
            .iter()
            .flat_map(|record| record.items.iter().map(|item| item.name.clone()))
            .collect()
    }

    pub fn collect_graph(&self) -> HashMap<String, Vec<String>> {
        self.0
            .iter()
            .flat_map(|record| record.graph.iter())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }*/
}

impl<'a> IntoIterator for &'a Records {
    type Item = <&'a Vec<Record> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Record> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.raw_data).into_iter()
    }
}
