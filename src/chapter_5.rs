pub fn sub_1() {
    let mut user1 = User {
        active: true,
        username: String::from("test@gmail.com"),
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("hello@world.com");

    let mut user2 = build_user(
        String::from("test@gmail.com"),
        String::from("test@gmail.com"),
    );

    let mut user3 = User {
        active: false,
        ..user1
    };

    println!("{:?}", user1.sign_in_count); // This works because it's simple data type, but not user1.username because it was moved

    let color = Color(255, 255, 255);
    let Color(x, y, z) = color;
    println!("{x}")
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
