use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Profile {
    rollno: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    refresh: u32,
    profile: Profile,
}
