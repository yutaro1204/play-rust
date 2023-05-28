#[derive(Debug)]
struct BasicTest {
  name: String,
}

#[derive(Debug)]
struct Object {
  name: String,
}

impl BasicTest {
  fn set_up(&self) {
    println!("setup")
  }

  fn tear_down(&self) {
    println!("tearDown")
  }
}

impl Object {
  fn tested_method(&self) -> u32 {
    1 + 1
  }
}

fn main() {
  let test = BasicTest { name: String::from("first test") };
  let object = Object { name: String::from("first test object") };
  println!("Run {}", test.name);
  test.set_up();
  println!("Test object {}", object.tested_method());
  // TODO panic になったら catch する
  assert!(object.tested_method() == 2);
  test.tear_down();
}
