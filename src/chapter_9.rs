use std::fs::File;

pub fn sub_1() {
    // let v = vec![1, 2, 3];

    // v[99];

    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(file) => println!("{:?}", file),
        Err(err) => println!("{:?}", err),
    }
}
