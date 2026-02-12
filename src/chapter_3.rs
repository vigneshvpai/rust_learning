const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn app() {
    let mut x = 5;
    println!("This is the value of {x}");
    x = 6;
    println!("This is the value of {x}");
    println!("{THREE_HOURS_IN_SECONDS}");
}
