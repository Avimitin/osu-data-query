use anyhow::{Context, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use regex::Regex;

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
        &self.error
    }
}

#[derive(Debug)]
pub struct BeatmapQuery<'a> {
    mode: &'a str,
    set: Option<String>,
    beatmap: Option<String>,
    key: &'a str,
}

impl<'a> BeatmapQuery<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            key,
            mode: "0",
            set: None,
            beatmap: None,
        }
    }

    pub fn set(mut self, set_id: impl Into<String>) -> Self {
        self.set = Some(set_id.into());
        self
    }

    pub fn mode(mut self, mode: &'a str) -> Self {
        self.mode = mode;
        self
    }

    pub fn beatmap(mut self, bid: impl Into<String>) -> Self {
        self.beatmap = Some(bid.into());
        self
    }

    pub fn from(key: &'a str, link: &'a str) -> Option<Self> {
        let re = Regex::new(r"https://osu.ppy.sh/beatmapsets/([0-9]+)#([a-z]+)/([0-9]+)").unwrap();

        if let Some(cap) = re.captures(link) {
            let sid = cap.get(1)?.as_str();
            let mode = cap.get(2)?.as_str();
            let bid = cap.get(3)?.as_str();
            Some(
                Self {
                    key,
                    mode,
                    set: Some(sid.into()),
                    beatmap: Some(bid.into()),
                }
            )
        } else {
            None
        }
    }

    pub async fn query(self) -> Result<Response> {
        let beatmap_api_url = format!("{}/{}", super::API_END_POINT, "get_beatmaps");

        let mut params = vec![("k", self.key), ("m", self.mode)];

        if let Some(bid) = &self.beatmap {
            params.push(("b", &bid));
        }

        // If beatmap id is not provided, try to use set id
        if self.beatmap.is_none() {
            if let Some(sid) = &self.set {
                params.push(("s", &sid));
            }
        }

        let url = Url::parse_with_params(&beatmap_api_url, &params)
            .with_context(|| format!("fail to build url with param: {:?}", self))?;

        let resp = reqwest::get(url).await?.json::<Response>().await?;
        Ok(resp)
    }
}
