use scraper::ElementRef;

pub async fn save_web_icon(url: &str, save_path: &str) -> bool {
    let url = url::Url::parse(url);
    if url.is_err() {
        return false;
    }
    let url = url.unwrap();
    let url = url.scheme().to_string() + "://" + url.host_str().unwrap();
    let fav_url = url.to_string() + "/favicon.ico";
    let ret = get_url_bin(&fav_url).await;
    if ret.is_some() {
        let ret = ret.unwrap();
        std::fs::write(save_path, ret).unwrap();
        return true;
    }
    let ret = get_url_text(&url).await;
    if ret.is_none() {
        return false;
    }
    let text = ret.unwrap();
    let (ret, link) = {
        let document = scraper::Html::parse_document(&text);
        let selector =
            scraper::Selector::parse("link[rel='icon'],link[rel='shortcut icon']").unwrap();
        let select: Vec<ElementRef> = document.select(&selector).collect();
        if select.len() == 0 {
            (false, "".to_string())
        } else {
            let ret = select[0].value().attr("href");
            if ret.is_none() {
                (false, "".to_string())
            } else {
                (true, ret.unwrap().to_string())
            }
        }
    };

    if !ret {
        return false;
    }
    let ret = get_url_bin(&link).await;
    if ret.is_none() {}
    std::fs::write(save_path, ret.unwrap()).unwrap();
    return true;
}

pub async fn get_url_bin(url: &str) -> Option<Vec<u8>> {
    let w = reqwest::get(url).await;
    if w.is_err() {
        return None;
    }
    let resp = w.unwrap();
    if resp.status().is_success() {
        let bytes = resp.bytes().await.unwrap();
        let buf = Vec::from(bytes);
        return Some(buf);
    } else {
        return None;
    }
}

pub async fn get_url_text(url: &str) -> Option<String> {
    let w = reqwest::get(url).await;
    if w.is_err() {
        return None;
    }
    let resp = w.unwrap();
    if resp.status().is_success() {
        let text = resp.text().await;
        if text.is_err() {
            return None;
        }
        return Some(text.unwrap());
    } else {
        return None;
    }
}
