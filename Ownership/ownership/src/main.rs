//Ownership, Borrowing and References

//Ownership
//C, C++ -> Meneory Management Control Issue
//Garbage Collector solved this issue, but created a new issue
//[stopping/Resuming the program]

//Rust solves this issue by using Ownership
//Ownership Rules
//1. Each value in Rust has a variable that’s called its owner.
//2. A value can only have one owner at a time.
//3. When the owner of a value goes out of scope, Rust will automatically free that value.
//4. The value will be freed when the owner goes out of scope.

//Summary
//Each value in Rust has a variable that's its ownder.
//There can be onlu one owner at a time.
//When the owner goes out of scope, the value will be freed.

fn main() {
    // Each value in Rust has a variable that's its owner.
    let s1 = String::from("Hello"); // s1 is the owner of the string
    // A value can only have one owner at a time.
    let s2 = s1; // s2 is now the owner of the string
    // s1 is no longer the owner of the string
    // println!("{}", s1); // This will cause a compile-time error
    // When the owner goes out of scope, the value will be freed.
    {
        let s3 = String::from("World"); // s3 is the owner of the string
        println!("{}", s3); // This will print "World"
    } // s3 goes out of scope here and the string is freed

    //reference borrowing
    calculate_length(&s2); // s2 is borrowed, not owned
    println!("{}", s2); // This will cause a compile-time error
    // s2 is still the owner of the string
    //println!("{}", s3); // This will cause a compile-time error s3 is out of scope

}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    // s is borrowed, not owned
    s.len()
}