use crate::user;
use anyhow::{anyhow, Context, Result};
use reqwest::{get, Url};

pub async fn get_users<'a>(k: &'a str, u: &'a str) -> Result<Vec<user::User>> {
    let api_url = format!("{}/{}", super::API_END_POINT, "get_user");
    let url = Url::parse_with_params(&api_url, vec![("k", k), ("u", u)])
        .with_context(|| "Fail to parse params".to_string())?;

    let resp = get(url.as_str())
        .await?
        .json::<user::Response>()
        .await
        .with_context(|| format!("Request {} fail", url.as_str()))?;

    match resp {
        user::Response::Error(e) => Err(anyhow!("{}", e.err())),
        user::Response::Succ(u) => Ok(u),
    }
}
