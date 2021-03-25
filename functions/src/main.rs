fn main() {
    println!("Hello, world!");
    another_function();
    arguments_in_function(5);
    multiple_arguments_in_function(5, 3.14159, "String");
    expression_or_statement();

    let result = return_a_value_implicitly(13, 13);
    println!("The value of result is {}.", result);

    let x = another_return_expression(13);
    println!("The value of x is {}", x);
}

fn another_function() {
    println!("Hello, world!");
}

fn arguments_in_function(x: i32) {
    println!("The value of x is {}", x);
}

fn multiple_arguments_in_function(x: u32, y: f32, z: &str) {
    println!("The value of x, y and z is {} {} {}, respectively.", x, y, z);
}

fn expression_or_statement() {
    let _y = 2;

    let x = {
        let y = 1;
        y + 1
    };
    println!("The value of y is {}", x);
}

fn return_a_value_implicitly(x: u32, y: u32) -> u32 {
    x * y
}

fn another_return_expression(x: i32) -> i32 {
    x + 1
}