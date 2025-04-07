// There are 5 Primitive Data Types
// int, float, bool, char and string

fn main() {

    //int
    // You get i (signed) and u (unsigned) types
    // i can be either + or - whereas u can only be +
    // The bits go from in to in-1 same for u in doubles starting from 8
    // That is i8 to i128 and u8 to u128 where the bits are n - 1
    // so for i8 - n = 8 therefore the size 2^(8-1) 
    let i: i32 = -127;
    let u: u32 = 564;
    //let z: u32 = -77; //this will not compile us cannot be -
    println!("i: {}, u: {}", i, u);

    //float
    // The type float is a floating point number and can be wither f32 or f64
    let f: f32 = 3.14;
    println!("Floating point: {}", f);

    //bool
    //Bool can be either true or false
    let b: bool = true;
    println!{"b is: {}", b}

    //char
    //Character - char
    let letter: char = 'a'; // single character
    println!("The letter is: {}", letter);

    //string
    //a sequence of characters
    let word: &str = "hello"; // string
    println!("The word is: {}", word);

}

