use crate::storage::DiskFileReader;
use index::Index;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use storage::FileReader;

mod index;
mod storage;

fn main() {
    let work_path = "/Users/kevinx/Codes/Rindex/";
    let mut handles = vec![];
    let index = Arc::new(Mutex::new(Index::new()));
    for i in 1..=4 {
        let index_clone = index.clone();
        let handle = thread::spawn(move || {
            let file_name = format!("{}/documents/document{}", work_path, i);
            let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
            let s = file_reader.read_as_string().unwrap();
            let _ = index_clone.lock().unwrap().add(i, &s);
        });
        handles.push(handle);
    }

    let index1 = Arc::new(Mutex::new(Index::new()));
    for i in 1..=2 {
        let index_clone = index1.clone();
        let handle = thread::spawn(move || {
            let file_name = format!("{}/documents/document{}", work_path, i);
            let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
            let s = file_reader.read_as_string().unwrap();
            let _ = index_clone.lock().unwrap().add(i, &s);
        });
        handles.push(handle);
    }

    let index2 = Arc::new(Mutex::new(Index::new()));
    for i in 3..=4 {
        let index_clone = index2.clone();
        let handle = thread::spawn(move || {
            let file_name = format!("{}/documents/document{}", work_path, i);
            let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
            let s = file_reader.read_as_string().unwrap();
            let _ = index_clone.lock().unwrap().add(i, &s);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    //handles.clear();

    let _ = index1
        .lock()
        .unwrap()
        .persist(format!("{}documents/index1", work_path).as_str());
    let _ = index2
        .lock()
        .unwrap()
        .persist(format!("{}documents/index2", work_path).as_str());

    let mut file_reader =
        DiskFileReader::from(format!("{}documents/index2", work_path).as_str()).unwrap();
    let index_string = file_reader.read_as_string().unwrap();
    let _ = index1.lock().unwrap().load(&index_string).unwrap();
    let k1 = index.lock().unwrap().map.keys().collect::<Vec<_>>();
    //k1.sort();
    let k2 = index1.lock().unwrap().map.keys().collect::<Vec<_>>();
    //k2.sort();

    assert_eq!(
        index.lock().unwrap().map.len(),
        index1.lock().unwrap().map.len()
    );
    /*assert_eq!(
        index.lock().unwrap().map.keys().collect::<Vec<_>>(),
        index1.lock().unwrap().map.keys().collect::<Vec<_>>()
    );*/
    //println!("{:?}", index);
}
