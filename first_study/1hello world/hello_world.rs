fn main(){
    // よくあるprint系関数。rust では関数では無くマクロなので
    // 貸出などを使うようになった際注意が必要
    println!("Hello world!");

    // {} の中に第２引数より後ろの値を代入することができる
    println!("Hello {}", "world!");

    // {0} と番号を入れることで対応する引数を代入することが可能
    println!("{0}です。初めまして{1}さん。", "私", "あなた");

    // {name} と入れることで対応する引数を代入することが可能
    println!("{name} 引数は名前を指定してとることが出来ます", name="println!の");

    // FIXZAM: 引数の数が足らないとエラー
    println!("FIXAM: {0}");

    // まだまだできることがあるので書きながら覚えるといいと思います
    // 参考: https://doc.rust-jp.rs/rust-by-example-ja/hello/print.html
}