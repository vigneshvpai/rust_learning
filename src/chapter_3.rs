const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn three_one() {
    let mut x = 5;
    println!("This is the value of {x}");
    x = 6;
    println!("This is the value of {x}");
    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;
    // Shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}")
    }
    println!("The value of x is {x}");

    // Shadowing
    // let spaces = "     ";
    // let spaces = spaces.len();

    // We can't mutate to different type
    // let mut spaces_mut = "     ";
    // spaces_mut = spaces_mut.len(); --> This won't work
}
