use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Database {
    dir: PathBuf,
    top_category: PaperCategory,
}

impl Database {
    pub fn new(dir: String) -> Self {
        Self {
            dir: PathBuf::from(dir.clone()),
            top_category: PaperCategory::new(vec![], PathBuf::from(dir)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaperCategory {
    relative_path: Vec<String>,
    dir: PathBuf,
    papers: PaperEntries,
    sub_categories: HashMap<String, PaperCategory>,
}

impl PaperCategory {
    pub fn new(relative_path: Vec<String>, dir: PathBuf) -> Self {
        Self {
            relative_path,
            dir,
            papers: vec![],
            sub_categories: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperEntry {
    pub id: String,
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub year: Option<u32>,
    // to be added
}

type PaperEntries = Vec<PaperEntry>;

/// Index of TermiPaper database
trait TpIndex {
    fn index_file(&self) -> PathBuf;
}

impl TpIndex for Database {
    fn index_file(&self) -> PathBuf {
        self.dir.join("index.termipaper.yml")
    }
}

impl TpIndex for PaperCategory {
    fn index_file(&self) -> PathBuf {
        self.dir.join("index.termipaper.yml")
    }
}

pub trait TpManage {
    fn add(&mut self, entry: PaperEntry);
}

impl TpManage for PaperCategory {
    fn add(&mut self, entry: PaperEntry) {
        self.papers.push(entry);
    }
}

impl TpManage for Database {
    fn add(&mut self, entry: PaperEntry) {
        // TODO: check category
        self.top_category.add(entry);
    }
}
