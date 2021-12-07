use crate::utils::read2;

const SAMPLES: &str = "assets/day2/commands.txt";

pub fn calculate_position() {
  let commands = read2(SAMPLES);

  let mut x = 0;
  let mut y = 0;

  match commands {
    Ok(commands) => {
      for i in 0..commands.len() {
        let (command, value) = &commands[i];

        match command.as_str() {
          "forward" => x += value,
          "down" => y += value,
          "up" => y -= value,
          _ => println!("Wrong command"),
        }
      }

      println!("X: {}    Y: {}    X*Y: {}", x, y, x * y);
    }

    Err(e) => println!("FailedL {:?}", e),
  }
}
