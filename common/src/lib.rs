pub mod config;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Records(Vec<Record>);

impl Records {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, record: Record) {
        self.0.push(record);
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(serde_json::to_writer(writer, self)?)
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn collect_items(&self) -> HashSet<String> {
        /*
        let mut items = HashSet::new();
        for record in &self.0.iter() {
            for item in &record.items {
                items.insert(item.name.clone());
            }
        }
        items*/
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
        /*
        let mut combined_graph = HashMap::new();
        for record in &self.0 {
            for (key, value) in &record.graph {
                combined_graph.insert(key.clone(), value.clone());
            }
        }
        combined_graph*/
    }
}

impl<'a> IntoIterator for &'a Records {
    type Item = <&'a Vec<Record> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Record> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}
