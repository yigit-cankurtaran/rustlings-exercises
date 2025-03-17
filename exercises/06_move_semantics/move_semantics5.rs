#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    // passing reference to not take ownership
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // removing reference to take ownership
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // prints "!"

    string_uppercase(data);
}
