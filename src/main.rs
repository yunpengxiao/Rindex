use crate::storage::DiskFileReader;
use anyhow::{Ok, Result};
use index::Index;
use std::sync::Mutex;
use std::thread;
use storage::FileReader;
mod index;
mod storage;

fn main() -> Result<()> {
    let work_path = "/Users/yunpeng.xiao/codes/index";
    let index = Mutex::new(Index::new());
    thread::scope(|s| {
        for i in 1..=4 {
            let index = &index;
            s.spawn(move || {
                let file_name = format!("{work_path}/documents/document{i}");
                let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
                let s = file_reader.read_as_string().unwrap();
                index.lock().unwrap().add(i, &s).unwrap();
            });
        }
    });

    let index1 = Mutex::new(Index::new());
    thread::scope(|s| {
        for i in 1..=2 {
            let index = &index1;
            s.spawn(move || {
                let file_name = format!("{work_path}/documents/document{i}");
                let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
                let s = file_reader.read_as_string().unwrap();
                index.lock().unwrap().add(i, &s).unwrap();
            });
        }
    });

    let index2 = Mutex::new(Index::new());
    thread::scope(|s| {
        for i in 3..=4 {
            let index = &index2;
            s.spawn(move || {
                let file_name = format!("{work_path}/documents/document{i}");
                let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
                let s = file_reader.read_as_string().unwrap();
                index.lock().unwrap().add(i, &s).unwrap();
            });
        }
    });

    index1
        .lock()
        .unwrap()
        .persist(format!("{work_path}/documents/index1").as_str())?;
    index2
        .lock()
        .unwrap()
        .persist(format!("{work_path}/documents/index2").as_str())?;

    let mut file_reader = DiskFileReader::from(format!("{work_path}/documents/index2").as_str())?;
    let index_string = file_reader.read_as_string()?;
    index1.lock().unwrap().load(&index_string)?;

    assert_eq!(index.lock().unwrap().map, index1.lock().unwrap().map);
    Ok(())
}
