/*
    # 実験ソースコード (rust) の実行方法

    各課題は, `works` モジュールに含まれているので, それぞれの関数を呼び出すことで実行できます.
    各課題を実行するためには, `main` 関数内で呼び出す関数を変更してください.

    ## install.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    ## run.
    ```bash
    cargo run
    ```

*/

mod works;
// use works::work_1_1;
// use works::work_1_2;
// use works::work_1_3;
// use works::work_1_4;
// use works::{work_2_1, work_2_2, work_2_3};
use works::{work_3_1, work_3_2, work_3_3, work_3_4};

fn main() {
  /*
    // 課題 1-1
    println!("work #1-1");
    work_1_1::main();

    // 課題 1-2
    println!("work #1-2");
    work_1_2::main();

    // 課題 1-3
    println!("work #1-3");
    work_1_3::main();

    // 課題 1-4
    println!("work #1-4");
    work_1_4::main();
  */

  /*
    // 課題 2
    println!("work #2");
    work_2_3::main();
  */

  // 課題 3-1
  println!("work #3-1");
  work_3_1::main();

  // 課題 3-2
  println!("work #3-2");
  work_3_2::main();

  // 課題 3-3
  println!("work #3-3");
  work_3_3::main();

  // 課題 3-4
  println!("work #3-4");
  work_3_4::main();
}
