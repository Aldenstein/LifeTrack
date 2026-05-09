use crate::models::User;

pub fn get_users() -> Vec<User> {
    let u1 = User {
        id: 1,
        name: String::from("Alice"),
    };

    let u2 = User {
        id: 2,
        name: String::from("Bob"),
    };

    vec![u1, u2]
}