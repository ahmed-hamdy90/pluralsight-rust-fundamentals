
#![allow(unused_variables)]

fn main() {
    // Explain Scope and Shadow in Rust
    let scope_variable1 = "outer scope: scope_variable1";
    println!("{}", scope_variable1);
    // Create a new scope using new Brackets, whatever function, version control
    {
        let scope_variable1 = "inner scope: scope_variable1";
        println!("{}", scope_variable1);
    }
    println!("{}", scope_variable1);

    // if print outer scope variable inside new scope will works,
    // as only inner scope variables will be dead after end scope
    let scope_variable2 = "outer scope: scope_variable2";
    {
        println!("{}", scope_variable2);
    }

    // We can ReDeclare a variable ans this will be variable shadowing
    let shadow_variable = "outer scope";
    let shadow_variable = 0;

    println!("{}", shadow_variable);
}
