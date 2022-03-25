fn main() {
    // 4.1 所有権とは？
    ownership_basic_sample();
    ownerships_and_function();
    return_value_and_scope();

    // 4.2 参照と借用
    reference_sample();
    // immutable_borrow();
    mutable_borrow();

    // 4.3 スライス型
    slice_sample();
}

fn ownership_basic_sample() {
    // 所有権のサンプル
    let s1 = String::from("hello");
    let s2 = s1;
    // s1の所有権がs2にムーブ deep copyではない
    // println!("{}, world!", s1); s1への参照は無効化されているのでコンパイルエラー
    println!("{}, world!", s2);
    // deep copyしたければclone()を使う
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
    // スカラー型はcopy traitが有効。所有権とはまた別
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn ownerships_and_function() {
    // 関数呼び出しでも、所有権はムーブする
    let s = String::from("hello");
    takes_ownership(s);

    // 所有権はtakes_ownershipに移動しているので下のprintln!()はコンパイルエラー
    // println!("{}, world", s);

    // xはコピーされるので、
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    // このprintln!()後、drop()が自動的に呼び出され破棄される
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    // some_integerがスコープを抜けるだけ
    println!("{}", some_integer);
}

fn return_value_and_scope() {
    // gives_ownerships()で生成した文字列の所有権がs1にムーブする
    let s1 = gives_ownership();
    println!("{}, world", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // s2の所有権はtakes_and_gives_back()の実行後にs3へムーブしている
    // そのためs2にアクセスしようとしてもコンパイルエラーとなる
    // println!("{}, people", s2);
    println!("{}, people", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn reference_sample() {
    let s1 = String::from("hello");
    // 引数として、変数の所有権でなく参照を渡す形にしている
    // 引数で参照を使うことを借用という
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // sはスコープ外になるが、所有権を持っているわけでないからdropされない
}

// fn immutable_borrow() {
//     let s = String::from("hello");
//     change(&s);
// }

// 不変参照の値を変更しようとすると、コンパイルエラーとなる
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn mutable_borrow() {
    let mut s1 = String::from("hello");
    // &mut を指定すると可変参照になる
    change(&mut s1);
    println!("{}", s1);

    // データ競合を回避するため、同じスコープの中では一つの変数に複数の可変参照を指定できない

    let mut s2 = String::from("hello world");
    let r1 = &mut s2;
    // r2の可変参照を生成しようとすると失敗する
    // let r2 = &mut s2;
    println!("{}", r1);
    // println!("{}, {}", r1, r2);

    // スコープをまたげば、同じ変数に対して複数の可変参照を作成できる
    let mut s3 = String::from("hello");
    {
        // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
        let r1 = &mut s3;
    }
    let r2 = &mut s3;

    // 不変参照と可変参照は組み合わせられない
    let mut s4 = String::from("hello");
    let r1 = &s4;
    let r2 = &s4;
    // この行でコンパイルエラー
    // let r3 = &mut s4;

    {
        // スコープをまたげば指定できる
        let r1 = &mut s4;
    }
}

fn change(some_string: &mut String) {
    some_string.push_str(", mutable borrow world");
}

fn dangling_pointer() {
    // let reference_to_nothing = dangle();
    let reference = no_dangle();
}

// sはスコープを抜けた後dropされる。そこへの参照を返そうとするためコンパイルエラー。
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    // 実体を返せば所有権をムーブできる
    s
}

fn slice_sample() {
    // Stringの一部（slice）への参照を取得できる
    let s = String::from("hello world");

    // こうとも指定できる
    // let hello = &s[..5];
    let hello = &s[0..5];
    // let world = &s[6..];
    let world = &s[6..11];

    let mut s = String::from("hello world");
    let word = first_word(&s);

    // 不変参照が指定されているので、ここでは変更できない
    // s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn literal_is_slice() {
    let my_string = String::from("hello world");
    let word = first_word_from_literal(&my_string[..]);

    let my_string_literal = "hello world";
    let word = first_word_from_literal(my_string_literal);
}

fn first_word_from_literal(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
