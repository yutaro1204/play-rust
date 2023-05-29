
#[derive(Debug)]
struct TestObject {
  name: String,
}

impl TestObject {
  fn tested_method(&self) -> u32 {
    1 + 1
  }
}

#[derive(Debug)]
struct TestSuite {
  name: String,
}

trait TestFrame {
  fn set_up(&self);
  fn tear_down(&self);
  fn run(&self, f: &Fn()) -> bool;
}

// test トレイトを実装して main で run を実行し、その結果が期待値と同じかどうかを検証する
// mock の注入などはできそうにない
// impl = テスト対象の struct の impl で、こいつがテスト用トレイトを実装する
// テスト用トレイトを実装した impl の set_up や tear_down はその　 struct 固有のフィールドのみを扱える
impl TestFrame for TestSuite {
  fn set_up(&self) {
    // 動的に struct に変数を持たせられないか
    println!("setup")
  }

  fn tear_down(&self) {
    println!("tearDown")
  }

  fn run(&self, f: &Fn()) -> bool {
    &self.set_up();
    let func = (self.get_func)(self, "set_up");
    println!("Method {}", func);
    f();
    // let object = Object { name: String::from("first test object") };
    // println!("Test object {}", object.tested_method());
    // return assert!(object.tested_method() == 2);
    &self.tear_down();
    return true
  }
}

fn function() {
  println!("I'm a function!");
}

fn main() {
  let test = TestSuite { name: String::from("first test") };
  let test_object = TestObject { name: String::from("test object") };
  println!("Run {}", test.name);
  // let f = test_object.tested_method;
  let result = test.run(&function);
  // 関数を引数で渡す
  // https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/input_functions.html
  // https://rustwiki.org/en/rust-by-example/fn/closures/input_parameters.html
  println!("Result {}", result);
}


// memo
// self.get_func というメソッドがあるようで、これを利用すれば struct とメソッド名を run メソッドに渡すことで
// テスト対象のメソッドを呼び出しできるのではないだろうか
// https://docs.rs/wasmtime/latest/wasmtime/struct.Instance.html
// https://stackoverflow.com/questions/37370120/right-way-to-have-function-pointers-in-struct

// ひとまずの目標: run メソッドで外部から渡した関数を呼び出し、それをアサーションした結果を return する
// set_up で値を定義し、それを利用したメソッドを実行し、それをアサーションした結果を変数に保存してから tear_down を実行し、その後 run の引数として結果を return する
