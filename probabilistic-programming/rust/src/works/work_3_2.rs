use super::work_3_1::checkout_counter_sim;

pub fn main() {
  let lambda = 2.0;
  let mus = vec![1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5];
  let s = 1;
  let n = 100_000;
  let mut results = vec![];

  for &mu in &mus {
    let loss_rate = checkout_counter_sim(lambda, mu, s, n);
    results.push((mu, loss_rate));
    println!("mu: {}, loss rate: {}", mu, loss_rate);
  }

  // csv に出力
  let mut wtr = csv::Writer::from_path("results_3_2.csv").unwrap();
  wtr.write_record(&["mu", "loss_rate"]).unwrap();
  for &(mu, loss_rate) in &results {
    wtr
      .write_record(&[mu.to_string(), loss_rate.to_string()])
      .unwrap();
  }
  wtr.flush().unwrap();
}
