use crate::auth_utils;

pub mod models {
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
}

pub fn login(cred: models::Credentials) {
    super::database::get_user();
}

pub fn authenticate(cred: models::Credentials) {
    if let super::database::Status::Connected = super::database::connect_to_database() {
        auth_utils::login(cred);
    }
}
