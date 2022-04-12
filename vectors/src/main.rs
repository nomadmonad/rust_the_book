fn main() {
    push_elements();
    read_elements();
    use_enum_to_push_different_types();
}

fn push_elements() {
    let mut v = Vec::new();

    v.push(3);
    v.push(4);
    v.push(5);

    // このメソッドを抜けると、vとvの中の要素は全てドロップされる
}

fn read_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[1];

    // 不変借用しているので、このタイミングで変更を加えるとコンパイルエラー
    // v.push(6);
    println!("The first element is {}", first);
    // ここで変更を加える場合はOK
    v.push(6);
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn use_enum_to_push_different_types() {
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
}
