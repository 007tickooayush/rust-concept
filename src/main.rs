struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("cornell@deg.com"),
        username: String::from("cornell"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;

    user1.username = String::from("cornell_1");


}