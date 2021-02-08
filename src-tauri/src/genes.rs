use gedcom::{parse, GedcomData};
use std::path::PathBuf;

pub fn init() -> GedcomData {
  println!("Attempting init... {:?}", std::env::current_dir());

  let mut path: PathBuf = std::env::current_dir().unwrap();
  path.push("../fixtures/washington.ged");
  println!("relative: {:?}", &path);

  let absolute_path: PathBuf = std::fs::canonicalize(path).unwrap();
  println!("absolute: {:?}", absolute_path);

  let ged = std::fs::read_to_string(absolute_path).unwrap();

  parse(ged.chars())
}
