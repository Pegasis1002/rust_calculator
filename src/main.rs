mod func;
use text_io::read;

fn main() {
    println!("This is a calculator!");
    println!("Please enter a number: ");
    let num1: i64 = read!();
    println!("what operation?\n");
    let op: char = read!();
    println!("Please enter second number: ");
    let num2: i64 = read!();

    let mut result: i64 = 0;
    if op == '+' {
        result = func::math_funcs::add(num1, num2);
    } else if op == '-' {
        result = func::math_funcs::sub(num1, num2);
    } else if op == '*' {
        result = func::math_funcs::multiply(num1, num2);
    } else if op == '/' {
        result = func::math_funcs::divide(num1, num2);
    } else if op == '%' {
        result = func::math_funcs::remender(num1, num2);
    } else if op == '^' {
        result = func::math_funcs::pow(num1, num2);
    }
    println!("ans: {}", result);
}
