fn main() {
    // Declare the first variable binding with the name "shadow_num"
    let shadow_num = 5;

    // Declare a second variable binding
    // This binding shadows the existing variable named "shadow_num"
    let shadow_num = shadow_num + 5;

    println!("The number is {}.", shadow_num);

    // Declare a third variable binding
    // This binding shadows the second binding of the variable named "shadow_num"
    {
        let shadow_num = shadow_num * 2;
        println!("The number inside block is {}.", shadow_num);
    }
    
    println!("The number is {}.", shadow_num);

    if (true) {
        let shadow_num = shadow_num * 2;
        println!("The number inside IF is {}.", shadow_num);
    }

    println!("The number is {}.", shadow_num);
}
