
use async_trait::{async_trait};
use chrono::Local;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::collections::HashMap;
use thiserror::Error;

//####################################################
// ↓↓↓↓↓↓↓　Interface Base class ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
//####################################################

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("[ERROR] NotFound(404 Not Found: code({0}), url({1})")]
    NotFound(String, String),

    #[error("[ERROR] not 200 http return : code({0}), url({1})")]
    InterfaceException(String, String),

    #[error("[ERROR] Failed to send a request: {0}")]
    SendRequest(#[source] reqwest::Error),

    #[error("[ERROR] Failed to read the response body: {0}")]
    ResponseBody(#[source] reqwest::Error),
    
    //#[error("Failed to get chart_url")]
    //StdError(#[from] Box<dyn std::error::Error + 'static>),
    
    // #[error("Failed to make the link URL absolute")]
    // AbsolutizeUrl(#[source] url::ParseError),
}
// 抽象化（InterfaceはError, sendを実装する必要がある）
#[async_trait]
pub trait Interface: Sync + Send {

    // コンストラクタ
    fn new() -> Self ;

    // デフォルトは何もしない。パラメータがあれば各IFで実装
    fn add_param(&mut self, _params: HashMap<&str, &str>) {
        // Default
    }
    // HTTPリクエスト送信
    async fn send_request(&mut self) -> Result<(), ApiError>;
    
    // HTMLXMLを解析し、必要なデータを抽出&contentへ格納
    fn on_parse(&mut self, httpxml: String);
    
    // contentの返却
    fn get_content(&self) -> HashMap<String, String>;

}

pub async fn send(url: &str) -> Result<String, ApiError> {
    make_log("[INFO]", "send_request", "reqwest::get start");
    // TODO 関数化したい
    let result = reqwest::get(url).await;
    let response = match result {
        Ok(result) => result,
        Err(e) => {
            return Err(ApiError::SendRequest(e));
        }
    };

    make_log("[INFO]", "send_request", "reqwest::analyze start");
    // Check if status is within 200-299.
    if response.status().is_success() {
        make_log("[INFO]", "send_request", "reqwest::text start");
        let text = response.text().await;
        let httpxml = match text {
            Ok(httpxml) => httpxml,
            Err(e) => {
                return Err(ApiError::ResponseBody(e));
            }
        };
        return Ok(httpxml);
    } else {
        // not 200 http return
        match response.status() {
            StatusCode::NOT_FOUND => {
                println!("error: 目的のページがありませんでした。");
                return Err(ApiError::NotFound(response.status().to_string(), url.to_string()));
            },
            _ => {
                println!("error: その他のエラーが発生しました。");
                return Err(ApiError::InterfaceException(response.status().to_string(), url.to_string()));
            }
        }
    }
}
//####################################################
// ↑↑↑↑↑↑↑↑　Interface Base class ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
//####################################################

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