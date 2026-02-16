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

pub fn sub_2() {
    // let width = 30;
    // let height = 50;

    // let rect = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Area of rectangle is {} square pixels", rect1.area());

    println!("Rectangle is {rect1:#?}");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sqr1 = Rectangle::square(30);
    println!("Rectangle is {sqr1:#?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
