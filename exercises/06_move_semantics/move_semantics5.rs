#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) {
    let uppercase_data = data.to_uppercase();

    println!("{uppercase_data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Last character: {last_char}");

    string_uppercase(data);
}