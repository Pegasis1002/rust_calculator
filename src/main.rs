use text_io::read;

fn main() {
    println!("This is a calculator!");
    let input: i32 = read!();
    println!("{}", input + 1);
}
