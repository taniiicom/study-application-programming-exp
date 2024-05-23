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

// 100回 の独立なシミュレーションを実行し, S_1000 の平均値と分散を計算
fn simulate_and_calculate(n: usize, p: f64, d: i32, simulations: usize) -> Result<(f64, f64), Box<dyn Error>> {
  let mut s1000_values = Vec::new();

  for _ in 0..simulations {
    let walk = random_walk(n, p, d);
    s1000_values.push(walk[n]); // S1000の値を収集
  }

  let mean = s1000_values.iter().copied().sum::<i32>() as f64 / simulations as f64;
  let variance = s1000_values.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / simulations as f64;

  Ok((mean, variance))
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let n = 1000;
  let p = 0.5;
  let d = 1;
  let simulations = 100;

  // 1 回のランダムウォークを保存
  let walks = random_walk(n, p, d);
  let filename = "random_walk.csv";
  save_walk_to_csv(&walks, filename)?;

  // 100 回のシミュレーションで S_1000 の平均値と分散を計算
  let (mean, variance) = simulate_and_calculate(n, p, d, simulations)?;
  println!("S1000の平均値: {:.3}", mean);
  println!("S1000の分散: {:.3}", variance);

  Ok(())
}
