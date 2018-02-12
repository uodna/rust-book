fn main() {
    mutation1();
    mutation2();

    constants();

    shadowing1();
    shadowing2();
    shadowing3();
}

fn mutation1() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn mutation2() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // error
    println!("The value of x is: {}", x);
}

fn constants() {
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadowing1() {
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);
}

fn shadowing2() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn shadowing3() {
    let mut spaces = "   ";
    spaces = spaces.len(); // error
}
