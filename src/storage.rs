use std::io::{Read, SeekFrom};
use std::{fs::File, io::Seek};

use thiserror::Error;

pub type Result<T> = core::result::Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait FileWritter {
    fn write(&self, buf: &String) -> Result<()>;
}

pub trait FileReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<()>;

    fn read_as_string(&mut self) -> Result<String>;
}

#[derive(Debug)]
pub struct DiskFileWritter;

impl FileWritter for DiskFileWritter {
    fn write(&self, buf: &String) -> Result<()> {
        Ok(())
    }
}

pub struct DiskFileReader {
    pub document: File,
    pub offset: usize,
}

impl DiskFileReader {
    pub fn from(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        Ok(DiskFileReader {
            document: file,
            offset: 0,
        })
    }
}

impl FileReader for DiskFileReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.document.seek(SeekFrom::Start(self.offset as u64))?;
        let read_bytes = self.document.read(buf)?;
        self.offset += read_bytes;
        Ok(())
    }

    fn read_as_string(&mut self) -> Result<String> {
        let mut result = String::new();
        let _ = self.document.read_to_string(&mut result);
        Ok(result)
    }
}
