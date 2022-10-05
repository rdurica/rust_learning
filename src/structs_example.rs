struct User {
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn use_struct() {
    let mut user1 = User {
        email: String::from("john.doe@example.com"),
        username: String::from("jdoe"),
        is_active: true,
        sign_in_count: 1,
    };

    // user1 is mutable so you can "change" email
    user1.email = String::from("anotheremail@example.com");

    // Creating a new User instance using one of the values from user1
    let user2 = User {
        is_active: user1.is_active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // struct update syntax. ".." = remaining fields not explicitly set should have the same value as the fields in the given instance
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        is_active: true,
        username, // shorthand initialization same as "username: username"
        email,
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct_example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;
fn struct_without_fields() {
    let subject = AlwaysEqual;
}
