//====== Module declare ======
pub mod beatmaps;
pub mod user;

pub mod response;

pub mod config;
pub use config::AppConfig;

// ======= API =========
pub static API_END_POINT: &str = "https://osu.ppy.sh/api";

use anyhow::{anyhow, Result};
pub async fn get_beatmaps<'a>(
    k: &'a str,
    m: &'a str,
    s: &'a str,
    b: &'a str,
) -> Result<Vec<beatmaps::BeatMap>> {
    let res = beatmaps::get_beatmaps(beatmaps::BeatmapQuery::new(k, m, s, b)).await?;

    match res {
        beatmaps::Response::SuccResp(b) => Ok(b),
        beatmaps::Response::ErrorResp(e) => Err(anyhow!("{}", e.err())),
    }
}
