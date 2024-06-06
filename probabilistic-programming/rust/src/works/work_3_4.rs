use super::work_2_2::poisson_exp;
use rand_distr::{Distribution, Exp};

/// MMMinf のシミュレーション
pub fn mms_infinite_sim(
  lambda: f64, // 客の到着分布 (指数分布のパラメータ λ)
  mu: f64,     // サービスの所要時間分布 (指数分布のパラメータ λ)
  s: i32,      // サーバの数
  n: i32,      // 客の数
) -> f64 // ロス率
{
  let events = poisson_exp(lambda, n);
  let mut t: f64; // 客の到着時刻
  let mut t_s = vec![0.0; s as usize]; //  t_s[i] が i 番目のサーバの利用終了時刻を表す配列
  let mut sum_loss_time = 0.0; // 客の待ち時間の累計

  for event in events {
    t = event.time;

    // 最も早くサービスが終了するサーバを選ぶ
    let min_ts = t_s
      .iter_mut()
      .min_by(|a, b| a.partial_cmp(b).unwrap())
      .unwrap();

    if *min_ts < t {
      *min_ts = t + Exp::new(mu).unwrap().sample(&mut rand::thread_rng());
    } else {
      sum_loss_time += *min_ts - t;
    }
  }

  sum_loss_time / n as f64
}

pub fn main() {
  let lambdas = vec![1.0, 1.5, 2.0, 2.5, 3.0];
  let mus = vec![0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5];
  let s = 1;
  let n = 100_000;

  let mut wtr = csv::Writer::from_path("results_3_A.csv").unwrap();
  wtr
    .write_record(&["lambda", "mu", "average_wait_time"])
    .unwrap();

  for &lambda in &lambdas {
    for &mu in &mus {
      let avg_wait_time = mms_infinite_sim(lambda, mu, s, n);
      println!("lambda: {}, mu: {}, average wait time: {}", lambda, mu, avg_wait_time);
      wtr
        .write_record(&[lambda.to_string(), mu.to_string(), avg_wait_time.to_string()])
        .unwrap();
    }
  }
  wtr.flush().unwrap();
}
