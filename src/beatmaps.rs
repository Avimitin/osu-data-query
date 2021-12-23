use anyhow::{Context, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatMap {
  pub beatmapset_id: String,
  pub beatmap_id: String,
  pub artist: String,
  pub artist_unicode: String,
  pub title: String,
  pub title_unicode: String,
  #[serde(rename = "difficultyrating")]
  pub stars: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
  SuccResp(Vec<BeatMap>),
  ErrorResp(ErrorResp),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResp {
  error: String,
}

impl ErrorResp {
  pub fn err(&self) -> &str {
    return &self.error;
  }
}

#[derive(Debug)]
pub struct BeatmapQuery<'a> {
  pub mode: &'a str,
  pub set: &'a str,
  pub beatmap: &'a str,
  key: &'a str,
}

impl<'a> std::default::Default for BeatmapQuery<'a> {
  fn default() -> Self {
    BeatmapQuery {
      key: "",
      mode: "0",
      set: "",
      beatmap: "",
    }
  }
}

impl BeatmapQuery<'_> {
  pub fn new<'a>(key: &'a str, mode: &'a str, set: &'a str, beatmap: &'a str) -> BeatmapQuery<'a> {
    return BeatmapQuery {
      key,
      mode,
      set,
      beatmap,
    };
  }
}

pub async fn get_beatmaps(qry: BeatmapQuery<'_>) -> Result<Response> {
  let beatmap_api_url = format!("{}/{}", super::API_END_POINT, "get_beatmaps");

  let mut params = Vec::new();
  params.push(("k", qry.key));

  if qry.mode.len() == 0 {
    // default use osu! std mode
    params.push(("m", "0"));
  } else {
    params.push(("m", qry.mode));
  }

  if qry.beatmap.len() != 0 {
    params.push(("b", qry.beatmap));
  }

  // If beatmap id is not provided, try to use set id
  if qry.beatmap.len() == 0 && qry.set.len() != 0 {
    params.push(("s", qry.set));
  }

  let url = Url::parse_with_params(&beatmap_api_url, &params)
    .with_context(|| format!("fail to build url with param: {:?}", qry))?;

  let resp = reqwest::get(url).await?.json::<Response>().await?;
  Ok(resp)
}
