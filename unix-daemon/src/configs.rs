use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub rollno: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub refresh: u64,
    pub profile: Profile,
    pub timeouts: u64,
}


