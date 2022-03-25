fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is: {}", number);

    let ary = [1, 2, 3, 4, 5];

    for element in ary.iter() {
        println!("value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("test result: {}", test(true));
}

fn test(cond: bool) -> i32 {
    if cond {
        4
    } else {
        6
    }
    println!("ここまで到達するかな？");
}
