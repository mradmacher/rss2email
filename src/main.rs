use regex::Regex;

fn main() {
    let re = Regex::new(r"(?s)<entry>(.+?)</entry>").unwrap();
    let title_re = Regex::new(r"<title(.*?)>(.+?)</title>").unwrap();
    let link_re = Regex::new(r#"<link href="(.+?)""#).unwrap();
    //let summary_re = Regex::new(r"(?s)<summary(.*?)>(.+?)</summary>").unwrap();
    let response = reqwest::blocking::get("https://www.hardscrabble.net/feed.xml").unwrap().text().unwrap();
    for cap in re.captures_iter(&response) {
        println!("{:?}", title_re.captures(&cap[1]).unwrap().get(2).map_or("", |m| m.as_str()));
        //println!("{:?}", summary_re.captures(&cap[1]).unwrap().get(2).map_or("", |m| m.as_str()));
        println!("{:?}", link_re.captures(&cap[1]).unwrap().get(1).map_or("", |m| m.as_str()));
    }
}
