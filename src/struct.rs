struct User {
    username: String,
    age: u32,
    active: bool,
}

fn main() {
  let username : String = String::from("Alice"); //structer data togerther fo accessing and storing data in a single unit
  //tuple structure is a struct without named fields, and the fields are accessed by their position in the tuple.
    let user: User = User {
        username,
        age: 30,
        active: true,
    };

    println!("Username: {}, Age: {}, Active: {}", user.username, user.age, user.active);
}