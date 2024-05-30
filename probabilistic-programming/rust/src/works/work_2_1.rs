// event.rs

// イベント構造体
pub struct Event {
  pub id: i32,
  pub time: f64,
}

// Event 構造体のコンストラクタを実装
impl Event {
  pub fn new(id: i32, time: f64) -> Event {
    Event { id, time }
  }
}

// Event 構造体のデストラクタを実装
// (= Drop トレイトを実装)
impl Drop for Event {
  fn drop(&mut self) {
    // println!("Event {} dropped.", self.id);
  }
}
