pub fn run() {
    // Primitive:
    let _hello_primitive = "Hello";
    // String:
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
    println!("Length: {}", _hello_primitive.len());
    
    hello = String::from("BYE! :O");
    println!("Length (2): {}", hello.len());

    // -------------------------------------------------------------------
    // Constants:
    // 1. you aren’t allowed to use mut with constants. Constants aren’t just immutable by default —they’re always immutable
    // 2. the type of the value must be annotated.
    // 3. Constants can be declared in any scope, including the global scope
    // 4. constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS : {}", MAX_POINTS);

    // -------------------------------------------------------------------
    // Shadowing:
    // Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used:

    fn shadow_fn() {
        let x = 5;
        let x= x + 1;
        let x= x * 2;
        
        println!("The value of x is : {}", x)
    }
    shadow_fn();

    // Shadowing is different from marking a variable as mut , 
    // because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. 
    // By using let , we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, 
    // we can change the type of the value but reuse the same name:
    let spaces = "   ";
    let spaces = spaces.len(); // ok
    
    // let mut spaces = "   ";
    // spaces = spaces.len(); // NOT ok! wWe’re not allowed to mutate a variable’s type!
    // -------------------------------------------------------------------
    // Data Types:
}
