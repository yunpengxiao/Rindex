use crate::storage::{DiskFileWritter, FileReader, FileWritter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

//pub type Hit = Vec<u8>;
pub type Result<T> = core::result::Result<T, IndexError>;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization Error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hit {
    pub document_id: u32,
    pub location: Vec<u32>,
}

impl Hit {
    pub fn from(document_id: u32) -> Self {
        Self {
            document_id,
            location: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Index {
    pub map: HashMap<String, Vec<Hit>>,
    pub index_writter: IndexWritter<DiskFileWritter>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            map: HashMap::new(),
            index_writter: IndexWritter {
                writter: DiskFileWritter {},
            },
        }
    }

    // Index a new document
    pub fn add(&mut self, document_id: u32, text: &String) -> Result<()> {
        let text = text.to_lowercase();
        let words = Self::tokenize(&text);

        let mut s_map: HashMap<String, Hit> = HashMap::new();
        for (i, token) in words.iter().enumerate() {
            let hit = s_map
                .entry(token.to_string())
                .or_insert(Hit::from(document_id));
            hit.location.push(i as u32);
        }

        for (k, v) in s_map {
            let hits = self.map.entry(k).or_insert(Vec::new());
            hits.push(v);
        }

        Ok(())
    }

    pub fn merge(&mut self, other: Index) -> Result<()> {
        Ok(())
    }

    // Read the particular index
    pub fn get(&self, term: &String) -> Option<&Vec<Hit>> {
        self.map.get(term)
    }

    // Save the current index on the disk
    pub fn persist(&self, path: &str) -> Result<()> {
        let serialized_index = serde_json::to_string(&self.map)?;
        self.index_writter.write_index(&serialized_index, path)?;
        Ok(())
    }

    // Remove the particular document from the index
    pub fn remove(&mut self, document_id: u16) -> Result<()> {
        Ok(())
    }

    fn tokenize(text: &str) -> Vec<&str> {
        text.split(|ch: char| !ch.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .collect()
    }
}

#[derive(Debug)]
pub struct IndexWritter<T: FileWritter> {
    pub writter: T,
}

impl<T: FileWritter> IndexWritter<T> {
    pub fn write_index(&self, index: &String, path: &str) -> Result<()> {
        self.writter.write(index, path);
        Ok(())
    }
}

pub struct DocumentReader<T: FileReader> {
    pub reader: T,
}

impl<T: FileReader> DocumentReader<T> {
    pub fn read_document(&mut self, buf: &mut [u8]) -> Result<()> {
        self.reader.read(buf);
        Ok(())
    }
}
