use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    pub fn new_from_index(dir: String) -> Self {
        let mut db = Self::new(dir.clone());
        let index = db.index_from_file();
        match index {
            Ok(index) => {
                db.top_category.papers = index.papers;
                db.top_category.sub_categories = index.sub_categories;
                db
            }
            Err(_) => {
                eprintln!("Error: failed to load index file of database at '{}'.", dir);
                std::process::exit(1);
            }
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
            papers: HashMap::new(),
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
            papers: HashMap::new(),
            sub_categories: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperEntry {
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub year: Option<u32>,
    // to be added
}

type PaperID = String;
type PaperEntries = HashMap<PaperID, PaperEntry>;

/// Index of TermiPaper database
trait TpIndex {
    fn index_file(&self) -> PathBuf;

    fn index_from_file(&self) -> Result<Index, ()> {
        let index_file = self.index_file();
        if index_file.exists() {
            let index_str = std::fs::read_to_string(&index_file).unwrap();
            match serde_yaml::from_str(&index_str) {
                Ok(index) => Ok(index),
                Err(e) => {
                    eprintln!(
                        "Error: failed to parse index file at '{}'.",
                        index_file.to_str().unwrap()
                    );
                    eprintln!("Error: {:?}", e);
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
    /// Add a paper entry to the category
    /// 
    /// If the paper entry is already in the category, it will be overwritten.
    fn add(&mut self, id: PaperID, entry: PaperEntry, force: bool) -> Result<(), Box<dyn Error>>;
}

impl TpManage for PaperCategory {
    fn add(&mut self, id: PaperID, entry: PaperEntry, force: bool) -> Result<(), Box<dyn Error>> {
        // 1. check if the paper entry is already in the category
        if self.papers.contains_key(&id) && !force {
            eprintln!("Error: the paper entry '{}' already exists in the category.", id);
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "paper entry already exists")));
        }
        // 2. add the paper entry to the category
        self.papers.insert(id, entry);
        // 3. save the index
        let index = Index {
            papers: self.papers.clone(),
            sub_categories: self.sub_categories.clone(),
        };
        self.index_to_file(&index)
    }
}

impl TpManage for Database {
    fn add(&mut self, id: PaperID, entry: PaperEntry, force: bool) -> Result<(), Box<dyn Error>> {
        // 1. add to the top category (TODO: check category)
        self.top_category.add(id, entry, force)
    }
}
