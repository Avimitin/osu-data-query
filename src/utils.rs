use regex::Regex;

pub fn parse_from_link(link: &str) -> Option<(&str, &str, &str)> {
  let re = Regex::new(r"https://osu.ppy.sh/beatmapsets/([0-9]+)#([a-z]+)/([0-9]+)").unwrap();
  match re.captures(link) {
    Some(cap) => {
      let sid = cap.get(1);
      if let None = sid {
        return None;
      }
      let mode = cap.get(2);
      if let None = mode {
        return None;
      }
      let bid = cap.get(3);
      if let None = bid {
        return None;
      }

      let sid = sid.unwrap().as_str();
      let bid = bid.unwrap().as_str();
      let mode = mode.unwrap().as_str();

      Some((sid, mode, bid))
    }
    None => None,
  }
}

#[test]
fn test_regex() {
  let song = "https://osu.ppy.sh/beatmapsets/896080#osu/1872396";
  let result = parse_from_link(song);
  assert_eq!(Some(("896080", "osu", "1872396")), result);
}
