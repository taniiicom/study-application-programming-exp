// main.rs
use std::fs::File;
use std::io::{self, Write};
use std::iter::repeat;

use super::work_2_1::Event;
use super::work_2_2::poisson_exp;

fn calc_probability(mut events: Vec<Event>, n: i32, filename: &str) -> io::Result<()> {
  let finished_time = events[events.len() - 1].time.ceil() as usize;
  let mut num_event_lst: Vec<i32> = repeat(0).take(finished_time).collect();

  // events の各要素の time の floor をとる
  events.iter_mut().for_each(|event| {
    event.time = event.time.floor();
    num_event_lst[event.time as usize] += 1;
  });

  let histogram_num_k: Vec<i32> = num_event_lst
    .iter()
    .filter(|&&k| k < 30)
    .fold(repeat(0).take(30).collect(), |mut acc, &k| {
      // 該当する場所に 1 を足して, 次に渡す ^^
      acc[k as usize] += 1;
      acc
    });

  num_event_lst
    .iter()
    .filter(|&&k| k >= 30)
    .for_each(|&k| println!("k = {} is out of range", k));

  // 確率 p_k に変換
  let histogram_p_k: Vec<f64> = histogram_num_k
    .iter()
    .map(|&k| k as f64 / finished_time as f64)
    .collect();

  // CSV に出力
  let mut file = File::create(filename)?;
  writeln!(file, "k,p_k")?;
  for i in 0..histogram_p_k.len() - 1 {
    writeln!(file, "{:?},{:?}", i, histogram_p_k[i])?;
  }

  Ok(())
}

pub fn main() {
  let lambda_values = [1.0, 1.5, 2.0];
  let n = 100_000;

  for &lambda in &lambda_values {
    let events = poisson_exp(lambda, n);

    let filename = format!("histogram_p_k_lambda_{}.csv", lambda);

    // 度数分布をCSVに保存
    if let Err(err) = calc_probability(events, n, &filename) {
      eprintln!("Error: {}", err);
    }

    println!("Events for λ = {} saved to {}", lambda, filename);
  }
}
