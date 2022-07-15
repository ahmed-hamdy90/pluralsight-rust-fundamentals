
#![allow(unused_variables)]

fn main() {
    // Define String as Slice and String variables, Slice is Immutable But String is Mutable
    let person_name_slice_1 = "Donald Mallard";
    let person_name_string_1 = person_name_slice_1.to_string();
    // another way for define String variable
    let person_name_string_2 = "Donald Mallard".to_string();
    // another way for define String variable
    let person_name_string_3 = String::from("Donald Mallard");

    // How to convert from String Variable to Slice
    let person_name_slice_2 = person_name_string_1.as_str();

    // How to get a pointer from String variable
    let person_name_string_pointer = &person_name_string_1;
}
