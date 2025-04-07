// main() is the entry point for the program
// a function / variables should be written in snake case (all letters in small letters, words separated by underscores)
// snake case: hello_world  [PREFERRED]
// kebab case: hello-world [NOT RECOMMENDED]


fn main() {
    hello_world();
    number(5.0);

    // Expressions and Statements

    // Expressions: Anything that returns a value
    // Example of an expression
    let _x: f64 = {
        let price: f64 = 100.0;
        let qty: f64 = 5.0;
        price * qty //infers a return value
    };

    println!("The value of x is {:?}", _x);

    // Example of a statement
    // Statements: Anything that does not return a value
    // Almost all statements in rust end with ;
    // let y = let x = 10;
    // Varibale declerations: let x = 5;
    // Function definitions: fn foo() {}
    // Control flow statements: if, match, while, for, else

    let _y: i32; // Declaration statement, does not return a value

    // A statement can also be an assignment, which does not return a value
    _y = 10;

    println!("The value of y is {:?}", _y);

    let sum: f64 = add_return(5.0, 10.0);
    println!("The sum is: {}", sum);
    println!("Inline execute add - The sum is: {}", add_return(5.0, 10.0)); // Directly printing the return value

    let mix: f64= mix(5.8, 10.11);
    println!("The mix is: {}", mix);
}

// Order of functions does not matter - Hoisting allowed
fn hello_world() {
    println!("Hello, world!");
}

// Function with parameters
fn number(x: f64){
    println!("The number is: {}", x);
}

// Function returning a value
fn add_return(x: f64, y:f64) -> f64 {
    x + y // return value
}

fn mix(number: f64, sum: f64) -> f64 {
    number/(sum*sum) 

}
