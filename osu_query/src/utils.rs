use regex::Regex;

pub fn parse_from_link(link: &str) -> Option<(&str, &str, &str)> {
    let re = Regex::new(r"https://osu.ppy.sh/beatmapsets/([0-9]+)#([a-z]+)/([0-9]+)").unwrap();

    if let Some(cap) = re.captures(link) {
        let sid = cap.get(1)?.as_str();
        let mode = cap.get(2)?.as_str();
        let bid = cap.get(3)?.as_str();

        Some((sid, mode, bid))
    } else {
        None
    }
}

#[test]
fn test_regex() {
    let song = "https://osu.ppy.sh/beatmapsets/896080#osu/1872396";
    let result = parse_from_link(song);
    assert_eq!(Some(("896080", "osu", "1872396")), result);
}
