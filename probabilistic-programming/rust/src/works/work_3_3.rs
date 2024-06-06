use super::work_3_1::checkout_counter_sim;

pub fn main() {
  let lambda = 2.0;
  let mus = vec![0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5];
  let s_values = vec![1, 2, 3, 4, 5];
  let n = 100_000;

  let mut wtr = csv::Writer::from_path("results_3_3.csv").unwrap();
  wtr.write_record(&["S", "mu", "loss_rate"]).unwrap();

  for &s in &s_values {
    for &mu in &mus {
      let loss_rate = checkout_counter_sim(lambda, mu, s, n);
      wtr
        .write_record(&[s.to_string(), mu.to_string(), loss_rate.to_string()])
        .unwrap();
      println!("S: {}, mu: {}, loss rate: {}", s, mu, loss_rate);
    }
  }
  wtr.flush().unwrap();
}
