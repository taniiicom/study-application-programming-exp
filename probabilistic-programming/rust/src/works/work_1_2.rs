extern crate csv;
extern crate rand;

use rand::Rng;
use std::error::Error;
use std::fs::File;

fn rnd_exp(lambda: f64, n: usize) -> Vec<f64> {
  let mut rng = rand::thread_rng();

  // 生成する乱数の個数分だけイテレートし, 指数分布に従う乱数を生成する
  (0..n)
    .map(|_| {
      let u: f64 = rng.gen(); // [0, 1) の範囲で乱数を生成

      // 指数分布の累積分布関数 F(x) = 1 - e^(-λx) の逆関数 F^(-1)(u) = -ln(1 - u) / λ
      -u.ln() / lambda // 逆関数法により指数分布に従う乱数に変換
    })
    .collect()
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
