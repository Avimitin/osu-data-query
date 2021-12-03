use anyhow::{anyhow, Context, Result};
use reqwest::{get, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: String,
    username: String,
    playcount: String,
    pp_rank: String,
    level: String,
    accuracy: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    error: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Succ(Vec<User>),
    Error(Error),
}

pub async fn get_users<'a>(k: &'a str, u: &'a str) -> Result<Vec<User>> {
    let api_url = format!("{}/{}", super::API_END_POINT, "get_user");
    let url = Url::parse_with_params(&api_url, vec![("k", k), ("u", u)])
        .with_context(|| format!("Fail to parse params"))?;

    let resp = get(url.as_str())
        .await?
        .json::<Response>()
        .await
        .with_context(||{
            format!("Request {} fail", url.as_str())
        })?;

    match resp {
        Response::Error(e) => Err(anyhow!("{}", e.error)),
        Response::Succ(u) => Ok(u),
    }
}
