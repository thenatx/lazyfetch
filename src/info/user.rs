use std::env;

pub fn current() -> String {
    env::var("USER").expect("Error while try getting the user name")
}
