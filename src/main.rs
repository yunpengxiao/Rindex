use crate::storage::DiskFileReader;
use index::Index;
use storage::FileReader;

mod index;
mod storage;

fn main() {
    let work_path = "/Users/yunpeng.xiao/codes/index/";
    let mut index = Index::new();
    for i in 1..=4 {
        let file_name = format!("{}/documents/document{}", work_path, i);
        let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
        let s = file_reader.read_as_string().unwrap();
        let _ = index.add(i, &s);
    }

    let mut index1 = Index::new();
    for i in 1..=2 {
        let file_name = format!("{}/documents/document{}", work_path, i);
        let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
        let s = file_reader.read_as_string().unwrap();
        let _ = index1.add(i, &s);
    }
    let _ = index1.persist(format!("{}documents/index1", work_path).as_str());

    let mut index2 = Index::new();
    for i in 3..=4 {
        let file_name = format!("{}/documents/document{}", work_path, i);
        let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
        let s = file_reader.read_as_string().unwrap();
        let _ = index2.add(i, &s);
    }
    let _ = index2.persist(format!("{}documents/index2", work_path).as_str());

    let mut file_reader =
        DiskFileReader::from(format!("{}documents/index2", work_path).as_str()).unwrap();
    let index_string = file_reader.read_as_string().unwrap();
    let _ = index1.load(&index_string).unwrap();
    let mut keys1 = index.map.keys().collect::<Vec<_>>();
    keys1.sort();
    let mut keys2 = index1.map.keys().collect::<Vec<_>>();
    keys2.sort();

    assert_eq!(index.map.len(), index1.map.len());
    assert_eq!(keys1, keys2);
    //println!("{:?}", index);
}
