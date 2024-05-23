extern crate csv;
extern crate rand;

use rand::Rng;
use std::error::Error;
use std::fs::File;

fn random_walk(n: usize, p: f64, d: i32) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  let mut position = 0;
  let mut positions = Vec::new();
  positions.push(position);

  for _ in 0..n {
    if rng.gen_bool(p) {
      position += d;
    } else {
      position -= d;
    }
    positions.push(position);
  }

  positions
}

fn save_walk_to_csv(data: &Vec<i32>, filename: &str) -> Result<(), Box<dyn Error>> {
  let file = File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  for &value in data {
    wtr.write_record(&[value.to_string()])?;
  }
  wtr.flush()?;
  Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let n = 1000;
  let p = 0.5;
  let d = 1;
  let walks = random_walk(n, p, d);

  // CSV に保存
  let filename = "random_walk.csv";
  save_walk_to_csv(&walks, filename)?;

  Ok(())
}
