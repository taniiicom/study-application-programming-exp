use rand::Rng;

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

pub fn main() {
  let lambda = 2.0; // Set the lambda value
  let n = 10; // Number of random numbers to generate

  let random_numbers = rnd_exp(lambda, n);

  println!("Generated exponential random numbers:");
  for num in random_numbers {
    println!("{}", num);
  }
}
