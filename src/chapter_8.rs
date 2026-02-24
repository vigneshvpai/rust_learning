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
