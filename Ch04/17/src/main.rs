
fn main() {
    // how to sign variables, with initial type and without
    let integer_variable_with_type: u32 = 0; // unsigned integer

    let integer_variable_without_type = 0; // default declaration for zero is signed integer

    let float_variable = 1.0; // default declaration for float value is signed float

    // add prefix _ on variable to telling compiler skip un-used variable without warning
    let _unused_variable = 33;
}
