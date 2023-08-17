fn main() {
    let a = 10;
    let b = 2;

    printText("Multiplication");

    let mut result = &a * &b;
    println!("{}", result);

    println!("Addition");

    result = &a + &b;
    println!("{}", result);

    println!("Substraction");

    result = &a - &b;
    println!("{}", result);

    println!("Division");

    result = &a / &b;
    println!("{}", result);
}
fn printText(s: &str) {
    println!("{}", s);
}
