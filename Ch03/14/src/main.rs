
#![allow(unused_variables)]

fn main() {
    // Create two String Slices
    let duck = "Duck";
    let airlines = "Airlines";

    // Then concat them with different ways (Notice: all concatenation result give String variables from Strings Slices)
    let airlines_name_1 = [duck, " ", airlines].concat();
    println!("{}", airlines_name_1);
    let airlines_name_2 = format!("{}, {}", duck, airlines);

    // There another way for create String and begin edit it to works as concat Strings Slices
    let mut slogan = String::new();
    slogan.push_str("We hit the ground"); // add a string
    slogan.push(' '); // add character
    slogan = slogan + "every time";

    println!("{}", slogan)
}
