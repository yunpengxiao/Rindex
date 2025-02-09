use crate::storage::DiskFileReader;
use index::Index;
use storage::FileReader;

mod index;
mod storage;

fn main() {
    let mut index = Index::new();
    for i in 1..=4 {
        let file_name = format!("/Users/kevinx/Codes/Rindex/documents/document{}", i);
        let mut file_reader = DiskFileReader::from(file_name.as_str()).unwrap();
        let s = file_reader.read_as_string().unwrap();
        let _ = index.add(i, &s);
    }
    println!("{:?}", index);
    let _ = index.persist("/Users/kevinx/Codes/Rindex/documents/index");
}
