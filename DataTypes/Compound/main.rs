//Compound Types
// arrayes, tuples. clices and strings (slice string considered as primitive)

// Arrayes
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    //println!("Array: {}", numbers); will not compile cannot be Display format

    //let mix = [1, 2, "apples", 4, true]; will not compile array contents must be of same type

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits: {}", fruits[0]); // prints apple
    println!("Fruits: {}", fruits[1]); // prints banana
    println!("Fruits: {}", fruits[2]); // prints cherry
}