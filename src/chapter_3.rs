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

pub fn three_two() {
    let x = 92_888;
    let y = 0xff;
    let z = 0o77;

    println!("{x} and {y} and {z}");

    let x = 0b1111_0000;
    let y = b'#';

    println!("{x} and {y} ");

    let x = 2.0;
    let y: f32 = 3.14;

    println!("{x} and {y} ");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum} & {difference} & {product} & {quotient} & {truncated} & {remainder}");

    let t = true;
    let f = !t;

    println!("{t} and {f}");

    // Single quotes for char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");

    // Compound Types
    // Tuples
    let profile = ('V', 28, 171.8);
    let (x, y, z) = profile;
    println!("{x} and {y} and {z}");

    let age = profile.1;
    println!("{age}");

    let unit_value = ();

    let a = [1, 2, 3, 4, 5];
}
