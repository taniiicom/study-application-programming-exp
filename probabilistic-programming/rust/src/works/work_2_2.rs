// poisson.rs
use rand::Rng;

use super::work_2_1::Event;

fn rnd_exp(lambda: f64, n: i32) -> Vec<f64> {
  let mut rng = rand::thread_rng();

  // 生成する乱数の個数分だけイテレートし, 指数分布に従う乱数を生成する
  (0..n)
    .map(|_| {
      let u: f64 = rng.gen(); // [0, 1) の範囲で乱数を生成

      // 指数分布の累積分布関数 F(x) = 1 - e^(-λx) の逆関数 F^(-1)(u) = -ln(1 - u) / λ
      -(1. - u).ln() / lambda // 逆関数法により指数分布に従う乱数に変換
    })
    .collect()
}

pub fn poisson_exp(lambda: f64, n: i32) -> Vec<Event> {
  (0..n)
    .scan(0.0, |t: &mut f64, i| {
      let rnd = rnd_exp(lambda, 1);
      *t += rnd[0];
      Some(Event::new(i, *t))
    })
    .collect()
}
