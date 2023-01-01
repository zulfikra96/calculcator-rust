use std::env::{args, Args};
fn main() {
    let mut args:Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0);
    let second = args.nth(0).unwrap();
    let result: f64 = match operator {
        Some(res) => {
            let first_number: f64 = first.parse::<f64>().unwrap();
            let second_number: f64 = second.parse::<f64>().unwrap();
            if res == "+" {
                first_number + second_number
            } 
            else if res == "*" {
                first_number * second_number
            } 
            else if res == "/" {
                first_number / second_number
            } 
            else if res == "-" {
                first_number - second_number
            }
            else {
                panic!("Invalid operator")
            }
        },
        None => panic!("")
    };
    println!("Result is {} ", result);
}
