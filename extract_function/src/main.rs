fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest_number(&number_list);

    // 最大値は{}です
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = largest_number(&number_list);

    // 最大値は{}です
    println!("The largest number is {}", largest);
}

fn largest_number(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}
