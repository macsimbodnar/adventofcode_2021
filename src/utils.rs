use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read(path: &str) -> Result<Vec<i32>, Error> {
  let file = File::open(path)?;

  let br = BufReader::new(file);
  br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
    .collect()
}

// pub fn read2<R: Read>(io: R) -> Result<Vec<i32>, Error> {
//   let br = BufReader::new(io);
//   let mut v = vec![];
//   for line in br.lines() {
//     v.push(
//       line?
//         .trim()
//         .parse()
//         .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
//     );
//   }
//   Ok(v)
// }

// fn main() -> Result<(), Error> {
//     let vec = read(File::open("/some/path/to/file")?)?;
//     // use `vec` for whatever
//     Ok(())
// }
