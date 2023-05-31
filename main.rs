
#[derive(Debug)]
struct TestObject {
    name: String,
}

enum TestMethodName {
    TestedMethod
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
    fn run(&self, f_name: TestMethodName, object: &TestObject);
}

// test トレイトを実装して main で run を実行し、その結果が期待値と同じかどうかを検証する
// mock の注入などはできそうにない
// impl = テスト対象の struct の impl で、こいつがテスト用トレイトを実装する
// テスト用トレイトを実装した impl の set_up や tear_down はその　 struct 固有のフィールドのみを扱える
impl TestFrame for TestSuite {
    fn set_up(&self) {
        println!("setup")
    }

    fn tear_down(&self) {
        println!("tearDown")
    }

    fn run(&self, f_name: TestMethodName, object: &TestObject) {
        match f_name {
            TestMethodName::TestedMethod => {
                println!("Run TestObject.tested_method");
                &self.set_up();
                let result = object.tested_method();
                println!("Result {}", result);
                println!("Expected {}", 2);
                assert!(result == 2);
                &self.tear_down();
            },
            _ => println!("Run no method"),
        }
    }
}

fn main() {
  let test = TestSuite { name: String::from("first test") };
  let test_object = TestObject { name: String::from("test object") };
  println!("TestSuite {}", test.name);
  println!("TestObject {}", test_object.name);

  test.run(TestMethodName::TestedMethod, &test_object);

  println!("All tests run!");
}

// TODO
// 関数を引数で渡す
// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/input_functions.html
// https://rustwiki.org/en/rust-by-example/fn/closures/input_parameters.html
