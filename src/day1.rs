use crate::utils::read;

const SAMPLES: &str = "assets/day1/measurements.txt";

pub fn count_increases() {
  let measures = read(SAMPLES);

  match measures {
    Ok(measures) => {
      if measures.len() < 2 {
        println!("The size should be at least 2 ");
      }

      let mut counter = 0;
      for i in 1..measures.len() {
        if measures[i - 1] < measures[i] {
          counter += 1;
        }
      }

      println!(
        "Measurements that are larger than the previous measurement: {}",
        counter
      );
    }

    Err(e) => println!("Failed: {:?}", e),
  }
}

pub fn sliding_window_increases() {
  let measures = read(SAMPLES);

  match measures {
    Ok(measures) => {
      if measures.len() < 4 {
        println!("The size should be at least 2 ");
      }

      let mut counter = 0;
      for i in 3..measures.len() {
        let pre_sum = measures[i - 1] + measures[i - 2] + measures[i - 3];
        let now_sum = measures[i] + measures[i - 1] + measures[i - 2];
        if pre_sum < now_sum {
          counter += 1;
        }
      }

      println!("Sums that are larger than the previous sum: {}", counter);
    }

    Err(e) => println!("Failed: {:?}", e),
  }
}
