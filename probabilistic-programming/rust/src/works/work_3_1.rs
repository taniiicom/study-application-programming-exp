use super::work_2_2::poisson_exp;
use rand_distr::{Distribution, Exp};

/// チェックアウトカウンタのシミュレーション
pub fn checkout_counter_sim(
  lambda: f64, // 客の到着分布 (指数分布のパラメータ λ)
  mu: f64,     // サービスの所要時間分布 (指数分布のパラメータ λ)
  s: i32,      // サーバの数
  n: i32,      // 客の数
) -> f64 // ロス率
{
  let events = poisson_exp(lambda, n);
  let mut t: f64; // 客の到着時刻
  let mut t_s = vec![0.0; s as usize]; //  t_s[i] が i 番目のサーバの利用終了時刻を表す配列
  let mut num_loss = 0; // 到着したがサービスを諦めた客の数

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
      num_loss += 1;
    }
  }

  num_loss as f64 / n as f64
}

pub fn main() {
  let lambda = 2.0;
  let mu = 1.0;
  let s = 2;
  let n = 100_000;
  let loss_rate = checkout_counter_sim(lambda, mu, s, n);
  println!("Loss rate: {}", loss_rate);
}
