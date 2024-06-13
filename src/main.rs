#![feature(trace_macros)]
#![feature(log_syntax)]
mod decl_macros;
mod proc_macro;
mod smart_pointers;
#[macro_use]
mod greeting;

use crate::greeting::base_greeting_fn;

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
}
