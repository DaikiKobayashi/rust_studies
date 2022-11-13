fn main() {
    // 整数
    println!("1 + 2 = {}", 1i32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // FIXZAM: u32(unsind int32) はマイナス値をサポートしないためエラー
    // println!("1 - 2 = {}", 1u32 - 2);

    // 論理演算式
    println!("true and false = {}", true && false);
    println!("true or false = {}", true || false);
    println!("NOT true is = {}", !true);

    // bit 演算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);

    // bit シフト
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
