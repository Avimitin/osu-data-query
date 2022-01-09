use crate::{beatmaps, user, utils};
use anyhow::{anyhow, bail, Context, Result};
use reqwest::{get, Url};

pub async fn get_beatmaps<'a>(
    k: &'a str,
    m: &'a str,
    s: &'a str,
    b: &'a str,
) -> Result<Vec<beatmaps::BeatMap>> {
    let res = beatmaps::BeatmapQuery::new(k)
        .mode(m)
        .set(s)
        .beatmap(b)
        .query()
        .await?;

    match res {
        beatmaps::Response::SuccResp(b) => Ok(b),
        beatmaps::Response::ErrorResp(e) => Err(anyhow!("{}", e.err())),
    }
}

pub async fn get_beatmaps_from_link(k: &str, link: &str) -> Result<Vec<beatmaps::BeatMap>> {
    let result = utils::parse_from_link(link);
    if result.is_none() {
        bail!("Invalid URL {}", link);
    }
    let result = result.unwrap();
    let mode = match result.1 {
        "osu" => "0",
        "taiko" => "1",
        "fruits" => "2",
        "mania" => "3",
        _ => "0",
    };
    log::trace!("query params: {:#?}", result);
    get_beatmaps(k, mode, result.0, result.2).await
}

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
