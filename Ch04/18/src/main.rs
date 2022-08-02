
fn main() {
    // How Rust handle Casting variables data type
    let float_variable: f32 = 17.5;
    let unsigned_variable: u8 = 8;

    // This will throw an error
    // let divided_result = float_variable / unsigned_variable;

    // must casting one of variable first before get divided result
    let cast_unsigned_variable = unsigned_variable as f32;
    let dividend_result = float_variable / cast_unsigned_variable;

    println!("{}", dividend_result);

    // How Rust automatic Casting Signed integer type variable as string -> Using ASCII code
    let number: u8 = 65;
    // Not works if number was float or not Signed integer with 8 size like u128
    // let number:f32 = 12.9;
    // let number:u128 = 9;

    let letter = number as char;

    println!("{}", letter)
}
