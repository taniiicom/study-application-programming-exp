extern crate csv;
extern crate rand;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::error::Error;
use std::fs::File;

// 線形合同法による乱数生成
fn rnd_lcg(n: usize, seed: u64, a: u64, b: u64, m: u64) -> Vec<u64> {
  let mut results = Vec::with_capacity(n);
  let mut x = seed;

  for _ in 0..n {
    x = (a * x + b) % m;
    results.push(x);
  }

  results
}

// メルセンヌ・ツイスタによる乱数生成
fn rnd_mt(n: usize, seed: u64, m: u64) -> Vec<u64> {
  let mut rng = StdRng::seed_from_u64(seed);
  let mut results = Vec::with_capacity(n);

  for _ in 0..n {
    results.push(rng.gen_range(0..m));
  }

  results
}

// CSVファイルに保存
fn save_to_csv(data: &Vec<(u64, u64)>, filename: &str) -> Result<(), Box<dyn Error>> {
  let file = File::create(filename)?;
  let mut wtr = csv::Writer::from_writer(file);

  for &(x, y) in data {
    wtr.write_record(&[x.to_string(), y.to_string()])?;
  }
  wtr.flush()?;
  Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let n = 100_000;
  let seed = 42;
  let a = 1664525;
  let b = 1013904223;
  let m = 2_147_483_647;

  // 線形合同法による乱数生成
  let lcg_numbers = rnd_lcg(n, seed, a, b, m);

  // メルセンヌ・ツイスタによる乱数生成
  let mt_numbers = rnd_mt(n, seed, m);

  // 乱数の組を作成
  let lcg_pairs: Vec<(u64, u64)> = lcg_numbers.iter().zip(lcg_numbers.iter().skip(1)).map(|(&x, &y)| (x, y)).collect();
  let mt_pairs: Vec<(u64, u64)> = mt_numbers.iter().zip(mt_numbers.iter().skip(1)).map(|(&x, &y)| (x, y)).collect();

  // CSV に保存
  save_to_csv(&lcg_pairs, "lcg_pairs.csv")?;
  save_to_csv(&mt_pairs, "mt_pairs.csv")?;

  Ok(())
}
