//Borrowing and References
//Safety and Performance
//Borrowing and References are powerful concepts in Rust that allow you to access data without taking ownership of it.
// This is useful for performance reasons, as it allows you to avoid unnecessary copies of data.
//Borrowing allows you to create a reference to a value without taking ownership of it.
// This means that you can access the value without having to move it into a new variable.
//References are immutable by default, meaning that you cannot change the value they point to.
// However, you can create mutable references that allow you to change the value.
// This is useful for performance reasons, as it allows you to avoid unnecessary copies of data.
//References are also used to create slices, which are a view into a portion of an array or vector.
// This is useful for performance reasons, as it allows you to avoid unnecessary copies of data

//In Rust Saftey can be thought of as the prevention of certain bugs and errors that commonly occur in other languages
//Things like null pointer dereferencing - dangling pointers - buffer overlows - data races

//Understanding References
// You create references by borrowing from the intial owner of a value
// Can be Immutable and Immutable
//Refernce are created using the & operator
//Immutable references are created using the & operator
//Mutable references are created using the &mut operator
fn main() {
    let _x: i32 = 5;
    //let y: i32 = x; this transfers ownership of x to y which means x will no longer exist
    let _y: &i32 = &_x; // this creates a reference to x
    println!("x: {}, y: {}", _x, _y); // this will print x: 5, y: 5 

    //let z: &i32 = &mut _x; // this will not compile because you cannot create a mutable reference to an immutable value

    let mut _a: i32 = 5;
    let _b: &mut i32 = &mut _a; // this creates mutable reference
    *_b += 10; 
    println!("b: {}", _b); // this will print a: 15
    //println!("a: {}", _a); // this will cause compile error because _a is borrowed as mutable
    //When you try to use _a in println!, you're trying to read _a while it's already being changed by _b. 
    //Rust doesn't allow this to keep your data safe and avoid problems.

    //You can only have one mutable reference to a value in a particular scope. 
    let mut _c: i32 = 5;
    {
    let _d: &mut i32 = &mut _c; // this creates mutable reference
    *_d += 10; 
    println!("d: {}", _d);
    }

    //now you can use _c again
    println!("c: {}", _c); // this will print c: 15
    //You can have multiple immutable references to a value in a particular scope, but you cannot have mutable references at the same time.
}
