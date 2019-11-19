struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        username: String::from("someotherusername123"),
        email: String::from("someoneelse@example.com"),
        sign_in_count: 2,
        active: true,
    };

    user2.email = String::from("anotheremail@example.com");

    let user3 = build_user(
        String::from("example@example.com"),
        String::from("superuser123"),
    );

    let user4 = User {
        email: String::from("yetanotherexample@example.com"),
        username: String::from("anotherusername567"),
        active: user2.active,
        sign_in_count: user2.sign_in_count,
    };

    let user5 = User {
        email: String::from("yetanotherexample@example.com"),
        username: String::from("anotherusername567"),
        ..user3
    };

    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Colour(0, 0, 0);
    let origin = Point(1, 1, 1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
