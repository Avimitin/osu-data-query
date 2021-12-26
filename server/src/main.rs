#[macro_use]
extern crate rocket;
use osu_query::beatmaps::BeatMap;
use rocket::serde::json::Json;
use std::collections::HashMap;

#[get("/")]
fn index() -> Json<HashMap<String, String>> {
  let mut resp = HashMap::new();
  resp.insert(String::from("status"), String::from("ok"));
  Json(resp)
}

#[get("/get_beatmaps")]
fn get_beatmaps() -> Json<BeatMap> {
  let beatmap = BeatMap {
    artist: String::from("a"),
    artist_unicode: String::from("b"),
    beatmap_id: String::from("1"),
    beatmapset_id: String::from("2"),
    stars: String::from("3"),
    title: String::from("c"),
    title_unicode: String::from("d"),
  };
  Json(beatmap)
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, get_beatmaps])
}
