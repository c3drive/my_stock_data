
use async_trait::{async_trait};
use chrono::Local;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use thiserror::Error;
use std::collections::HashMap;
use url::Url;

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//####################################################
// ↓↓↓↓↓↓↓　Interface class ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
//####################################################

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("NotFound(404 Not Found)")]
    NotFound(),

    #[error("not 200 http return : StatusCode({0})")]
    InterfaceException(String),
    
    #[error("timeout error : url({0})")]
    TimeOutError(String),
    
    #[error("Failed to get connection")]
    ConncectionPoolError(#[from] reqwest::Error),

    // #[error("Failed to send a request")]
    // SendRequest(#[source] reqwest::Error),

    // #[error("Failed to read the response body")]
    // ResponseBody(#[source] reqwest::Error),
    
    #[error("Failed to get chart_url")]
    StdError(#[from] Box<dyn std::error::Error + 'static>),
    
    // #[error("Failed to make the link URL absolute")]
    // AbsolutizeUrl(#[source] url::ParseError),
}

pub struct Interface {
    state: Option<Box<(dyn State + 'static + Sync + Send)>>,
    url_base: String,
    url: String,
    content: String,
}
trait State {
    fn send_request(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)>;
    fn approve(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)>;
    fn content<'a>(&self, chart: &'a Interface) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn send_request(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)>  {
        self
    }
}
struct PendingReview {}
impl State for PendingReview {
    fn send_request(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)> {
        self
    }
    fn approve(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)> {
        Box::new(Published {})
    }
}
struct Published {}

impl State for Published {
    fn send_request(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)> {
        self
    }

    fn approve(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)> {
        self
    }
    fn content<'a>(&self, interface: &'a Interface) -> &'a str {
        &interface.content
    }
}
// 抽象化（InterfaceはError, sendを実装する必要がある）
#[async_trait]
//pub trait Interface<I, R>: Sync + Send {
pub trait InterfaceTrait<I, R>: Sync + Send {
    type Error;
    fn new(base_url: &str)-> I;
    fn add_param(&mut self, params: HashMap<String, String>);
    fn content(&self) -> &str;
    fn send_request(&mut self) ;
    fn approve(&mut self);
    //async fn send(&self)-> Result<(), Self::Error>;
    async fn send(&self)-> Result<R, Self::Error>;
    //fn on_parse(&self) -> String;
}

#[async_trait]
impl InterfaceTrait<Interface, InterfaceResponse> for Interface {
    //type Error = Box<dyn std::error::Error + 'static>;
    //type Error = Box<dyn ApiError + 'static>;
    type Error = ApiError;

    fn new(url_base: &str) -> Interface {
        Interface {
            state: Some(Box::new(Draft {})),
            url_base: String::from(url_base),
            url: String::from(url_base),
            content: String::new(),
        }
    }
    fn add_param(&mut self, params: HashMap<String, String>) {
        self.url.push_str(Url::parse_with_params(&self.url_base, params).unwrap().as_str());
    }
    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    fn send_request(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.send_request())
        }
    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    async fn send(&self) -> Result<(InterfaceResponse), Self::Error> {
    //async fn send(&self) -> Result<(), ApiError> {
        
        make_log("send", "start");
        //let url = format!("{}?s={}", self.url_base, self.param); // TODO 独自実装
        let url = &self.url_base; // TODO 独自実装

        make_log("send", "reqwest::get start");
        //println!("url: {}", url);

        if let Ok(res) = reqwest::get(url).await {
            // Check if status is within 200-299.
            if res.status().is_success() {
                let body = res.text().await?;

                // チャート画像抜き出し（1つしかない想定なので、一番最初のURLを使う）
                let links = get_links(&body, "https:".to_string())?; // TODO 独自実装
                let url = &links[0];
                //for link in links.iter() {
                    //chart_url = link;
                //     println!("chart: {}", link);
                //}
                let ret = InterfaceResponse::new(
                    String::from("success"),
                    body,
                    url,
                );
                //let tmp = GetStockChartResult::new(
                //    ret
                //);
                //return Box::new(std::future::Future<Output =ok(tmp));
                return Ok(ret);
            }
            
            // not 200 http return
            match res.status() {
                StatusCode::NOT_FOUND => {
                    println!("error: 目的のページがありませんでした。");
                    return Err(ApiError::NotFound());
                },
                _ => {
                    println!("error: その他のエラーが発生しました。");
                    return Err(ApiError::InterfaceException(res.status().to_string()));
                }
            }
        } else {
            println!("error: Webサーバーが見つかりませんでした。");
            return Err(ApiError::TimeOutError(url.to_string()));
        }
    }
}
// 抽象化（InterfaceはError, sendを実装する必要がある）
#[async_trait]
pub trait InterfaceResult {
    fn on_parse(&self) -> String;
}

#[derive(Debug)]
pub struct InterfaceResponse {
    response_code: String,
    response_body: String,
    chart_url: String, // TODO ここまで特化させるか検討だが、dobyもらったところでという問題はある
}
impl InterfaceResponse {
    // コンストラクタを提供することで構造体のフィールドはprivateのままとなる
    pub fn new(response_code: String, response_body: String, chart_url: &String) -> Self {
        InterfaceResponse {
            response_code: response_code,
            response_body: response_body,
            chart_url: String::from(chart_url),
        }
    }
    pub fn get_data(&self) -> &String {
        &self.response_body
    }
    pub fn get_url(&self) -> &String {
        &self.chart_url
    }
}

//####################################################
// ↑↑↑↑↑↑↑↑　Interface class ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
//####################################################

// ログ用文言を生成する関数
pub fn make_log(func: &str, message: &str) {
    let dt = Local::now();
    let time = dt.format("%Y年%m月%d日 %H:%M:%S").to_string();
    //format!("{} [function: {}], message: {} ", time, func, message)
    println!("{} [function: {}], message: {} ", time, func, message);
}

// サーバ時刻を指定フォーマットにして返却する関数
pub fn get_time() -> String {
    let dt = Local::now();
    dt.format("%Y年%m月%d日 %H:%M:%S").to_string()
}
pub fn get_links(body: &String, base_url :String) -> Result<Vec<String>, Box<dyn std::error::Error>>{
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

    Ok(links)
}