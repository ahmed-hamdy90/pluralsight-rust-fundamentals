#![allow(unused_variables)]

fn main() {
    // How to declare a Array, define type of values and size of array
    let location_as_array_1: [f32; 2];
    // Array With a default values
    let location_as_array_2: [f32; 2] = [41.409, -81.854];
    // Array Without predefine types, compiler to detect it from default values
    let location_as_array_3 = [41.409, -81.854];
    // Next way how to define array with default values for large size
    let location_as_array_4 = [0.0; 10000];

    // How to define a Tuple
    let location_as_tuple = ("KCLE", 41.409, -81.854);

    // How to get Values from Array/Tuple
    // First way with access value using index
    println!("Location name {}, latitude {}, longitude {}",
             location_as_tuple.0, location_as_tuple.1, location_as_tuple.0);
    // Second way with destructuring
    let (name, latitude, longitude) = location_as_tuple;
    println!("Location name {}, latitude {}, longitude {}", name, latitude, longitude);
}
