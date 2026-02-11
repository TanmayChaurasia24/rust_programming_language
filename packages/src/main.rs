use packages::*;

fn main() {
    let cred: auth_utils::models::Credentials = auth_utils::models::Credentials {
        username: String::from("tanmay"),
        password: String::from("tan444"),
    };
    auth_utils::authenticate(cred); // hiding the auth logic in lib.rs
}
