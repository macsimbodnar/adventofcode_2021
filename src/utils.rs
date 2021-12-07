use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read(path: &str) -> Result<Vec<i32>, Error> {
  let file = File::open(path)?;

  let br = BufReader::new(file);
  br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}

pub fn read2(path: &str) -> Result<Vec<(String, i32)>, Error> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let mut v = vec![];

  for line in br.lines() {
    let split: Vec<String> = line?.trim().split(' ').map(String::from).collect();

    if split.len() == 2 {
      let command = &split[0];
      let value = match split[1].parse::<i32>() {
        Ok(value) => value,
        Err(_e) => 0,
      };

      v.push((String::from(command), value));
    }
  }
  Ok(v)
}

// fn main() -> Result<(), Error> {
//     let vec = read(File::open("/some/path/to/file")?)?;
//     // use `vec` for whatever
//     Ok(())
// }
