fn main() {
    println!("Hello, world!");
    let x: i8 = 127;
    println!("The value of x is {}", x);
    // x += 2;
    println!("The value of x is {}", x);

    let f = 2.000_000_000_000_000_000_000_000_000_000_000_000_000_1;
    println!("The value of f is {}", f);

    let f2: f32 = 0.000_000_000_000_000_000_000_000_000_000_000_1;
    println!("The value of f2 is {}", f2);

    let a = 2 * 2;
    println!("The value of a is {}", a);

    let a = 2 + 2;
    println!("The value of a is {}", a);

    let a = 2 / 2;
    println!("The value of a is {}", a);

    let a = 2 - 2;
    println!("The value of a is {}", a);

    let a = 2 % 2;
    println!("The value of a is {}", a);

    // bool is 1 byte in size
    let t = true;
    println!("The value of t is {}", t);

    // char is 4 bytes in size
    let z = 'z';
    println!("The value of z is {}", z);

    // tuple
    let tup = (500, 3.1415, -1);
    println!("The value of tup is {:?}", tup); // pretty formatter
    println!("The value of tup is {:#?}", tup);  // pretty formatter
    println!("The value of the first element in tup is {}", tup.0);

    // destructuing
    let (x, y, z) = tup;
    println!("The value of x, y, z is {} {} {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is {:?}", a); 




    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{:?}", months);

    let odds: [i32; 5] = [1, 3, 5, 7, 9];
    println!("The value of odds is {:?}", odds);

    let ones = [1; 5];
    println!("The value of ones is {:?}", ones);

}
