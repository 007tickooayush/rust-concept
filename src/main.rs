// Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


// Tuple Struct
struct Point(i32,i32,u32);

fn main() {
    let mut user1 = User {
        email: String::from("cornell@deg.com"),
        username: String::from("cornell"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;

    user1.username = String::from("cornell_1");

    // create new user
    let mut user2 = build_user(String::from("jane@deg.com"), String::from("jane_doe"));

    user2.username = name;

    let user3 = User {
        email: String::from("creston@deg.com"),
        username: String::from("creston112"),
        ..user2 // get the remaining fields from struct user2
    };

    println!("user3 username: {}, email: {}, sign_in_count: {}, active: {}",user3.username, user3.email, user3.sign_in_count, user3.active);

    const POINT:Point = Point(-21,10,108);

    println!("Point: {},{},{}",POINT.0,POINT.1,POINT.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
