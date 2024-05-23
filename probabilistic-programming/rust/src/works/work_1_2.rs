extern crate csv;
extern crate rand;

use rand::Rng;
use std::error::Error;
use std::fs::File;

fn rnd_exp(lambda: f64, n: usize) -> Vec<f64> {
  let mut rng = rand::thread_rng();
  let mut results = Vec::new();

  for _ in 0..n {
    let u: f64 = rng.gen_range(0.0..1.0);
    let x = -1.0 * (1.0 / lambda) * (1.0 - u).ln();
    results.push(x);
  }

  results
}

fn calculate_mean_and_variance(data: &Vec<f64>) -> (f64, f64) {
  let n = data.len();
  let mean = data.iter().sum::<f64>() / n as f64;
  let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;

  (mean, variance)
}

fn save_to_csv(data: &Vec<f64>, filename: &str) -> Result<(), Box<dyn Error>> {
  let file = File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  for &value in data {
    wtr.write_record(&[value.to_string()])?;
  }
  wtr.flush()?;
  Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let lambdas = vec![1.0, 1.5, 2.0];
  let n = 10000;

  for &lambda in &lambdas {
    let random_numbers = rnd_exp(lambda, n);
    let (mean, variance) = calculate_mean_and_variance(&random_numbers);

    println!("λ = {}", lambda);
    println!("平均値: {:.3}", mean);
    println!("分散: {:.3}", variance);

    // CSV に保存
    let filename = format!("lambda_{}.csv", lambda);
    save_to_csv(&random_numbers, &filename)?;
  }

  Ok(())
}
