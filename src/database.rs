use serde::{Deserialize, Serialize};
use std::error::Error;
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
    sub_categories: Vec<String>,
}

impl PaperCategory {
    pub fn new(relative_path: Vec<String>, dir: PathBuf) -> Self {
        Self {
            relative_path,
            dir,
            papers: vec![],
            sub_categories: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub papers: PaperEntries,
    pub sub_categories: Vec<String>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            papers: vec![],
            sub_categories: vec![],
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

    fn index_from_file(&self) -> Result<Index, ()> {
        let index_file = self.index_file();
        if index_file.exists() {
            let index_str = std::fs::read_to_string(&index_file).unwrap();
            match serde_yaml::from_str(&index_str) {
                Ok(index) => index,
                Err(_) => {
                    eprintln!(
                        "Error: failed to parse index file at '{}'.",
                        index_file.to_str().unwrap()
                    );
                    Err(())
                }
            }
        } else {
            Ok(Index::new())
        }
    }

    fn index_to_file(&self, index: &Index) -> Result<(), Box<dyn Error>> {
        let index_file = self.index_file();
        let index_str = serde_yaml::to_string(&index)?;
        std::fs::write(index_file, index_str).map_err(|err| Box::new(err) as Box<dyn Error>)
    }
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
