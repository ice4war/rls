mod helper;
use helper::Mapper;
use std::fs;
use std::io::Result;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let root_dir = "/home/bharath/Downloads";
    let mut dirs: Vec<Directory> = vec![];
    for entry in WalkDir::new(root_dir).min_depth(1) {
        dirs.push(Directory {
            dirname: entry.unwrap(),
            size: 0.0,
            unit: "null".to_string(),
        });
    }
    for d in &mut dirs {
        // println!("{:?}, {:?}", d.dirname.depth(), d.dirname.path());
        d.start().unwrap();
    }
    for a in &dirs {
        println!(
            "\x1b[34m{0: <6} \x1b[32m{1: <5}  \x1b[33m{2}",
            a.size,
            a.unit,
            a.dirname.file_name().display()
        );
    }
}
#[derive(Debug)]
struct Directory {
    dirname: DirEntry,
    size: f64,
    unit: String,
}
impl Directory {
    fn start(&mut self) -> Result<()> {
        let mut total = 0;
        for sub_dir in WalkDir::new(self.dirname.path()) {
            let sub_dir = sub_dir.unwrap();
            if sub_dir.path().is_file() {
                let meta = fs::metadata(sub_dir.path())?;
                total += meta.len();
            }
        }
        let size = Mapper::convert_bytes(total as f64);
        self.size = size.0;
        self.unit = size.1;
        Ok(())
    }
}
