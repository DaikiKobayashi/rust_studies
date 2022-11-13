fn main() {
    // cargo c  - コンパイルエラーチェック
    // cargo t  - テスト実行 
    // cargo b  - デバッグビルド
    // cargo b -r   - リリースビルド

    // 変数定義
    let test_integer = 1;
    let test_integer2 = 1i32;
    let test_bigint = 1i64;

    // FIXZAM: 変数実初期化だとエラー
    let test_another_binding;
    println!("another binding: {}", test_another_binding);


    // println! macro printlnは関数では無くマクロなので注意
    println!("Hello, world!");
    println!("Hello, world! 1={}, 2={}, 3={}", "one", "two", 3i32);

    // TODOマクロ　残っている場合バックトレースを出力する
    todo!("TODO macro!");
    // print trace 
    // stack backtrace:
    // 0: std::panicking::begin_panic_handler
    // at /rustc/897e37553bba8b42751c67658967889d11ecd120/library\std\src\panicking.rs:584
    // 1: core::panicking::panic_fmt
    // at /rustc/897e37553bba8b42751c67658967889d11ecd120/library\core\src\panicking.rs:142
    // 2: study::main
    // at .\src\main.rs:5
    // 3: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
    // at /rustc/897e37553bba8b42751c67658967889d11ecd120\library\core\src\ops\function.rs:248
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // error: process didn't exit successfully: `target\debug\study.exe` (exit code: 101)
}
