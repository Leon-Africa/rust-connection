//Compound Types
// arrays, tuples. slices and strings (slice string (&str) considered as primitive)

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

    //Tuples
    // Different types of elements of fixed size
    let human = ("Alice", 30, false);
    println!("Tuple: {:?}", human);
    // let human: (&str, i32, bool) = ("Alice", 30, false); // for accuracy
    // let human: (String, i32, bool) = ("Alice", 30, to_string()); // String to string slice - will not compile without conversion

    let mix_tuple = ("Bob", 30, false, [1, 2,3,4,5]);
    println!("Mix Tuple: {:?}", mix_tuple);

    //Slices
    // Slices are a view into a contiguous [uninterrupted/adjacent] sequence of elements
    // Slices are not a type, but a reference to a contiguous sequence of elements

    //slices: [1,2,3,4,5] // no need to jump between memory locations
    let slice: &[i32] = &[1,2,3,4,5];
    println!("Slice: {:?}", slice);
    

    // A list of animal names stored as string slices (&str)
    // The string slices refer to string literals in the program's binary
    let animals: &[&str] = &["lion", "cheetah", "leopard"];
    println!("Animals: {:?}", animals);

    // A list of book names stored as String objects
    // Each String is a growable, heap-allocated string
    let letters: &[String] = &["Galatians".to_string(), "Ephesians".to_string(), "Philippians".to_string()];
    println!("Letters: {:?}", letters);
}