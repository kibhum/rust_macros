#![feature(trace_macros)]
#![feature(log_syntax)]
mod decl_macros;
mod proc_macro;
mod smart_pointers;
#[macro_use]
mod greeting;
#[macro_use]
mod generate_get_value;

use crate::greeting::base_greeting_fn;
mod account;
mod composing;

struct FirstName {
    value: String,
}
struct Pay {
    value: i32,
}
generate_get_value!(FirstName);
generate_get_value!(Pay, i32);
fn calculate_raise(first_name: String, _last_name: String, _age: i32, current_pay: i32) -> i32 {
    if first_name == "Sam" {
        current_pay + 1000
    } else {
        current_pay
    }
}

fn main() {
    let greet = greeting!("Sam", "Heya");
    println!("{}", greet);
    let greet_with_default = greeting!("Sam");
    println!("{}", greet_with_default);

    let first_raise = calculate_raise("Smith".to_string(), "Sam".to_string(), 20, 1000);
    println!("{}", first_raise);
    let second_raise = calculate_raise("Sam".to_string(), "Smith".to_string(), 1000, 20);
    println!("{}", second_raise);
    let name = FirstName {
        value: "Kibethh".to_string(),
    };
    println!("{}", name.get_value());
    account::transact();
}
