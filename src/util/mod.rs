use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn string_from_file(file_path: &'static str) -> io::Result<String> {
  let f = try!(File::open(file_path));
  let mut reader = BufReader::new(f);
  let mut buffer = String::new();

  reader.read_to_string(&mut buffer);
  Ok(buffer)
}
