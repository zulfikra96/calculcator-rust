use std::env::{args, Args};
fn main() {
    let mut args:Args = args();
    let first_number = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_number = args.nth(0).unwrap().parse::<f32>().unwrap();
    let result: f32 = operate(operator, first_number, second_number);
    println!("Result is {} ", result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' | ':' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used.")
    }
}