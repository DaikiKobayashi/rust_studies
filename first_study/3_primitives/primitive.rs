fn main(){
    // bool
    let logical: bool = true;
    // float
    let a_float: f64 = 1.0;
    // integer
    let an_integer = 5i32;

    // 明示的に指定しなかったっ場合、デフォルトを指定する
    let default_float = 3.0; // f64
    let default_integer = 1; // i32

    // 文脈から判定することも可能です。
    // この場合2回目の代入が i64 なので i64 で定義される
    let mut inferred_type = 1; 
    inferred_type = 123456789i64;

    // アンダースコアも可能
    println!("One million is written as {}", 1_000_000u32);

    // ミュータブル変数は値を変更することが可能
    let mut mutable = 1;
    mutable = 100;

    // FIXZAM: 変数の型を変えることは出来ない
    mutable = true; // ERROR

    // 変数はシャドーイングによって上書きすることができる
    let mutable = true; 
}