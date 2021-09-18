use chrono::Local;
use scraper::{Html, Selector};

// ログ用文言を生成する関数
pub fn make_log(log_type: &str, func: &str, message: &str) {
    let dt = Local::now();
    let time = dt.format("%Y年%m月%d日 %H:%M:%S:%f").to_string();
    //format!("{} [function: {}], message: {} ", time, func, message)
    println!("{} {} function: {}, message: {} ", time, log_type, func, message);
}

// サーバ時刻を指定フォーマットにして返却する関数
pub fn get_time() -> String {
    let dt = Local::now();
    dt.format("%Y年%m月%d日 %H:%M:%S").to_string()
}
pub fn get_links(body: &String, base_url :String) -> Vec<String> {
//pub fn get_links(body: String, base_url :String) -> Result<(), Box<dyn std::error::Error>>{
    //let response = self.client.get(url)?;
    //let base_url = response.url().clone();
    //let body = response.text()?;

    //let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#);
    // htmlのパース
    let document = Html::parse_document(&body);
    // cssセレクタのパース
    //let css = r#"img[class="chartimg" id="chartImg"]"#;
    let css = r#"img[class="chartimg"]"#;
    let selector = Selector::parse(css).unwrap();

    //let input = fragment.select(&selector).next().unwrap();
    //assert_eq!(Some("bar"), input.value().attr("value"));

    let mut links = Vec::new();
    //let url = base_url.join(input.value().attr("src"))?;

    // スクレイピング
    for node in document.select(&selector) {
        //　属性を取り出す
        let img_url = node.value().attr("src").unwrap();
        let url = base_url.clone() + img_url;
        links.push(url);
    }
    //"base_url" + input.value().attr("src"));
    

    // for src in doc.find(Name("img")).filter_map(|img| img.attr("src")) {
    //     match Url::parse(href) {
    //         Ok(url) => { links.push(url); },
    //         Err(UrlParseError::RelativeUrlWithoutBase) => {
    //             let url = base_url.join(src)?;
    //             links.push(url);
    //         },
    //         Err(e) => { println!("Error: {}", e); },
    //     }
    // }

    links
}