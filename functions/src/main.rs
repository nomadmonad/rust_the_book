fn main() {
    another_function(4, 5);

    let x = {
        let y = 6;
        y + 1
    };
    println!("x is: {}", x);

    let five = five();
    println!("five is: {}", five);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
