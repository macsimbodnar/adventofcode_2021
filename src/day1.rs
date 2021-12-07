use crate::utils::read;

pub fn count_increases() {
  let measures = read("assets/day1/measurements.txt");

  // Input should be at least size == 2
  match measures {
    Ok(measures) => {
      let mut counter = 0;
      for i in 1..measures.len() {
        let res;
        if measures[i - 1] < measures[i] {
          res = "increased";
          counter += 1;
        } else {
          res = "decreased";
        }
        println!("{} ({})", measures[i], res);
      }

      println!(
        "Measurements that are larger than the previous measurement: {}",
        counter
      );
    }

    Err(e) => println!("Failed: {:?}", e),
  }
}
