use std::collections::HashMap;

pub fn sub_1() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("{:?}", v3);

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[3 - 1];
    println!("The third element is {:?}", third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];

    println!("Value: {}", third);
    println!("Address stored in third (element): {:p}", third);
    println!("Address of third variable itself: {:p}", &third);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let s = String::from("hello");
    let mut s1 = s;
    s1.push('g');
    println!("{}", s1);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

pub fn sub_2() {
    let mut s = String::new();
    let s = "intial_content".to_string();
    let s = String::from("intial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let _s: &str = "hello";

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{s}")
}

pub fn sub_3() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(&s) => s,
        None => 0,
    };
}
