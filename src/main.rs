
use std::env::args;
fn main() {
    let mut args = args();
    let first_number : f32 = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let sec_number: f32 = args.nth(0).unwrap().parse::<f32>().unwrap();

    let result = operate( operator,  first_number,  sec_number);
    println!("{:?}", output(operator, first_number, sec_number, result));
}

fn operate(operator: char, first_number: f32, sec_number: f32) -> f32 {
    match operator {
        '+' => first_number + sec_number,
        '-' => first_number - sec_number,
        '*' | 'x' | 'X' => first_number * sec_number,
        '/' => first_number / sec_number,
        _ => panic!("Invalid operator"),
    }
}

fn output(operator: char, first_number: f32, sec_number: f32, result: f32) -> String {
    return format!("{} {} {} = {}",first_number, operator, sec_number, result)
}
