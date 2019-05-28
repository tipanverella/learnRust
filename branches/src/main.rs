fn main() {
    println!("Hello, world!");

    iflet();
}


fn iflet() {
    let condition : bool = true;
    let number : u8 = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
