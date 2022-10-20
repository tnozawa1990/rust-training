const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("the variable of x is {}", x);
    x = 6;
    println!("the variable of x is {}", x);

    tupple_test();

    another_funtion(5);

    println!("the variable of x is {}", five());
}

fn tupple_test() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}", y)
}

fn another_funtion(x: i32) {
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
