fn main() {
    println!("Hello, world!");
    let a_number = 10;
    let a_boolean = true;

    println!("The number is {}.", a_number);
    println!("The boolean is {}.", a_boolean);

    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1i32 + 2,
        8i32 - 5,
        15 * 3
    );

    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);
}
