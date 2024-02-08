use std::io::Read;

pub fn save_web_icon(url: &str, save_path: &str) -> bool {
    let url = url::Url::parse(url);
    if url.is_err() {
        return false;
    }
    let url = url.unwrap();
    let url = url.scheme().to_string() + "://" + url.host_str().unwrap();
    let fav_url = url.to_string() + "/favicon.ico";
    let ret = get_url_bin(&fav_url);
    if ret.is_some() {
        let ret = ret.unwrap();
        std::fs::write(save_path, ret).unwrap();
        return true;
    } else {
        let ret = get_url_text(&url);
        if ret.is_none() {
            return false;
        }
        let text = ret.unwrap();
        let document = scraper::Html::parse_document(&text);
        let selector =
            scraper::Selector::parse("link[rel='icon'],link[rel='shortcut icon']").unwrap();
        for ele in document.select(&selector) {
            if let Some(link) = ele.value().attr("href") {
                let ret = get_url_bin(link);
                if ret.is_none() {
                    continue;
                }
                std::fs::write(save_path, ret.unwrap()).unwrap();
                return true;
            }
        }
        return false;
    }
}

pub fn get_url_bin(url: &str) -> Option<Vec<u8>> {
    let w = reqwest::blocking::get(url);
    if w.is_err() {
        return None;
    }
    let mut resp = w.unwrap();
    if resp.status().is_success() {
        let mut buf = Vec::new();
        resp.read_to_end(&mut buf).unwrap();
        return Some(buf);
    } else {
        return None;
    }
}

pub fn get_url_text(url: &str) -> Option<String> {
    let w = reqwest::blocking::get(url);
    if w.is_err() {
        return None;
    }
    let resp = w.unwrap();
    if resp.status().is_success() {
        let text = resp.text();
        if text.is_err() {
            return None;
        }
        return Some(text.unwrap());
    } else {
        return None;
    }
}
