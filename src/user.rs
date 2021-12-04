use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct User {
    user_id: String,
    username: String,
    playcount: String,
    pp_rank: String,
    level: String,
    accuracy: String,
    country: String,
    pp_raw: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
"User {}'s data:
PC:    {}
RANK:  {}
LEVEL: {}
ACC:   {}
PP:    {}
",
            self.username, self.playcount, self.pp_rank, self.level, self.accuracy, self.pp_raw)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn err(&self) -> &str {
        &self.error
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Succ(Vec<User>),
    Error(Error),
}
