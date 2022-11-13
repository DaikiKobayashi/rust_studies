#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let でタプルの中を別の変数に束縛することが可能
    let (integer, boolean) = pair;

    // rust では最後に返す値は return を書かないのが習慣(書いても動くのかな?)
    (boolean, integer)
}

fn main() {
    // 複数のタプル宣言
    let mut long_tuple = (1u8, 2u16, 3u32, 4u64);
    
    // インデックスを用いて取り出すことができる
    let mut select_tuple = long_tuple.0;
    println!("select_tuple = {}", select_tuple);

    // タプルの中にタプルを書く
    let mut multi_tuple = ((1u32, 2u32), 3u32);
    // タプルは出力可能
    println!("multi_tuple = {:?}", multi_tuple);

    // FIXZAM: タプルの数が多すぎるとエラーになるらしい
    // println!("multi_tuple = {:?}", (1i32, 2i32, 3i32, 4i32, ...))

    let mut integer = 1u32;
    mutable(integer); // -> 関数の権限が渡した関数に移るためポインタの参照が切れる
}

fn mutable(mutable_primitive: u32){
    // あとこれ権限が移ってるからもとの関数じゃ使えない
    println!("mutableを付け無いと変数を上書きすることが出来ない {}", mutable_primitive);
}