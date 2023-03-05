fn main() {                                 // functions are statements
    println!("Hello, world!");

    test();
    test();
    add_numbers(20, 30);

    let result = add_float(6.6, 7.7);
    println!("{}", result);

    let number = {
        let x = 3;                          // expressions are everything else in rust that evaluate to something, anything
        x + 1                               // no semicolen becasue this returns to this "block" express
    };
    println!("{}", number);
}
 
fn test() {
    println!("Test has been called....")
}

fn add_numbers(x: i32, y: i32) -> i32{
    println!("the sum is: {}", x + y);
    let result = y - x;
    if result >= 10 {
        return result -10;
    }
    result
}

fn add_float(a: f32, b: f32) -> f32 {           // -> = return in rust
    a + b
}