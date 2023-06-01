
#[derive(Debug)]
struct TestObject {
    name: String,
}

impl TestObject {
    fn tested_method(&self) -> u32 {
        1 + 1
    }
}

struct TestSuite<'a> {
    name: String,
    closure: &'a dyn Fn() -> u32,
    expected: u32,
}

trait TestFrame {
    fn set_up(&self);
    fn tear_down(&self);
    fn assert_result(&self, result: u32, expected: u32);
    fn run(&self, f: &dyn Fn() -> u32, expected: &u32);
}

// test トレイトを実装して main で run を実行し、その結果が期待値と同じかどうかを検証する
// mock の注入などはできそうにない
// impl = テスト対象の struct の impl で、こいつがテスト用トレイトを実装する
// テスト用トレイトを実装した impl の set_up や tear_down はその　 struct 固有のフィールドのみを扱える
impl TestFrame for TestSuite<'_> {
    fn set_up(&self) {
        println!("setup")
    }

    fn tear_down(&self) {
        println!("tearDown")
    }

    fn assert_result(&self, result: u32, expected: u32) {
        // assertion の失敗時でもクラッシュせずに失敗したテスト数を取っておきたい
        assert!(result == expected)
    }

    fn run(&self, tf: &dyn Fn() -> u32, expected: &u32) {
        self.set_up();
        let result = tf();
        println!("Run {}", result);
        self.assert_result(result, *expected);
        self.tear_down();
    }
}

fn main() {
    let test_object = TestObject { name: String::from("test object") };
    println!("TestObject {}", test_object.name);

    let test_suite = TestSuite {
        name: String::from("first test"),
        closure: &|| test_object.tested_method(),
        expected: 2,
    };
    println!("TestSuite {}", test_suite.name);

    test_suite.run(&test_suite.closure, &test_suite.expected);

    println!("All tests run!");
}

// TODO
// 関数を引数で渡す
// https://doc.rust-jp.rs/rust-by-example-ja/fn/closures/input_functions.html
// https://rustwiki.org/en/rust-by-example/fn/closures/input_parameters.html


// dyn は &dyn にしないといけない？
// <'a> は何か？
// run や assert に u32 固定の値を渡しているが、肩が変動しても受け取れるようにしたい

// #[derive(Debug)] をつけられる場所とそうでない場所
// TestSuite に付けると、`(dyn Fn() -> u32 + 'a)` cannot be formatted using `{:?}` because it doesn't implement `Debug` と怒られる
