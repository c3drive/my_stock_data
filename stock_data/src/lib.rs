
use async_trait::{async_trait};
use chrono::Local;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::collections::HashMap;
use thiserror::Error;
use url::Url;

//####################################################
// ↓↓↓↓↓↓↓　Interface class ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
//####################################################

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("[ERROR] NotFound(404 Not Found: code({0}), url({1})")]
    NotFound(String, String),

    #[error("[ERROR] not 200 http return : code({0}), url({1})")]
    InterfaceException(String, String),
    
    // #[error("[ERROR] Failed to get connection: {0}")]
    // ConncectionPoolError(#[from] reqwest::Error),

    //#[error("Failed to get connection")]
    //ConncectionPoolError(),
    //#[error("An IO error occured: {0}")]
    //ConncectionPoolError(#[from] reqwest::Error),

    // #[error("Failed to get connection")]
    // //ConncectionPoolError{source: reqwest::Error},
    // ConncectionPoolError(),
    //#[error("Read error")]
    //ReadError { source: std::io::Error },
    //ConncectionPoolError(#[source] reqwest::Error),

    #[error("[ERROR] Failed to send a request: {0}")]
    SendRequest(#[source] reqwest::Error),

    #[error("[ERROR] Failed to read the response body: {0}")]
    ResponseBody(#[source] reqwest::Error),
    
    //#[error("Failed to get chart_url")]
    //StdError(#[from] Box<dyn std::error::Error + 'static>),
    
    // #[error("Failed to make the link URL absolute")]
    // AbsolutizeUrl(#[source] url::ParseError),
}

pub struct Interface {
    //state: Option<Box<InterfaceTrait>>,
    //state: Option<Box<(dyn InterfaceTrait<Interface, InterfaceResponse> + 'static + Sync + Send)>>,
    url_base: String,
    url: String,
    body: String,
}
// trait State {
//     fn send_request(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)>;
//     fn approve(self: Box<Self>) -> Box<(dyn State + 'static + Sync + Send)>;
//     fn content<'a>(&self, chart: &'a Interface) -> &'a str {
//         ""
//     }
// }
// struct SetUp {}

// impl InterfaceTrait<Interface, InterfaceResponse> for SetUp {

//      fn add_param<'a>(&self, infc: &mut Interface, params: HashMap<&str, &str>) {
//          infc.url = String::from(Url::parse_with_params(&infc.url_base, params).unwrap());
//      }
// }
// 抽象化（InterfaceはError, sendを実装する必要がある）
#[async_trait]
pub trait InterfaceTrait<I, R>: Sync + Send {
    //type ApiError;
    //fn new(base_url: &str)-> I;
    fn new(url_base: &str) -> Interface {
        Interface {
            //state: Some(Box::new(SetUp {})),
            url_base: String::from("url_base"),
            url: String::from(url_base),
            body: String::new(),
        }
    }
    fn add_param(&mut self, params: HashMap<&str, &str>);
    //fn add_param<'a>(&self, infc: &'a mut Interface, params: HashMap<&str, &str>) {
    //fn add_param<'a>(&self, infc: &mut Interface, params: HashMap<&str, &str>) {
    //fn add_param<'a>(&self, infc: &'a mut String, params: HashMap<&str, &str>) {
        //self.url.push_str(Url::parse_with_params(&self.url_base, params).unwrap().as_str());
        //infc.url = String::from(Url::parse_with_params(&infc.url_base, params).unwrap());
    //}
    async fn send_request(&mut self) -> Result<(), ApiError> ;
    
    fn on_parse(&mut self, body: String);
    
    fn get_data(&self) -> String;

    //fn get_data(&self) -> &str {
    // fn get_data<'a>(&self, infc: &'a Interface)  -> &'a str {
    //     //let infc: Interface = self.as_interface();
    //     &infc.body
    // }
}

#[async_trait]
impl InterfaceTrait<Interface, InterfaceResponse> for Interface {
//impl Interface {
    //type Error = Box<dyn std::error::Error + 'static>;
    //type Error = Box<dyn ApiError + 'static>;
    //type ApiError = ApiError;

    fn new(url_base: &str) -> Interface {
        Interface {
            //state: Some(Box::new(SetUp {})),
            url_base: String::from(url_base),
            url: String::from(url_base),
            body: String::new(),
        }
    }
    fn add_param(&mut self, params: HashMap<&str, &str>) {
        //self.url.push_str(Url::parse_with_params(&self.url_base, params).unwrap().as_str());
        //self.state.as_ref().unwrap().add_param(&self, params);
        self.url = String::from(Url::parse_with_params(&self.url_base, params).unwrap());
    }
    // async fn send_request(&mut self) -> Result<(), Self::Error> {
    async fn send_request(&mut self) -> Result<(), ApiError> {
        make_log("[INFO]", "send_request", "start");

        make_log("[INFO]", "send_request", "reqwest::get start");
        let result = reqwest::get(&self.url).await;
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
            let body = match text {
                Ok(body) => body,
                Err(e) => {
                    return Err(ApiError::ResponseBody(e));
                }
            };
            self.on_parse(body);
        } else {
            // not 200 http return
            match response.status() {
                StatusCode::NOT_FOUND => {
                    println!("error: 目的のページがありませんでした。");
                    return Err(ApiError::NotFound(response.status().to_string(), self.url.to_string()));
                },
                _ => {
                    println!("error: その他のエラーが発生しました。");
                    return Err(ApiError::InterfaceException(response.status().to_string(), self.url.to_string()));
                }
            }
        }

        make_log("[INFO]", "send_request", "end");
        return Ok(());
    }

    fn on_parse(&mut self, body: String) {
        make_log("[INFO]", "on_parse", "start");

        // やることがない
        &self.body.push_str(&body);
        // let ret = InterfaceResponse::new(
        //     String::from("success"),
        //     //body,
        //     String::from("body"),
        //     url,
        // );
        // //let tmp = GetStockChartResult::new(
        // //    ret
        // //);
        // //return Box::new(std::future::Future<Output =ok(tmp));
        make_log("[INFO]", "on_parse", "end");
    }

    fn get_data(&self) -> String {
        self.body.to_string()
    }
    // fn get_data<'a>(&self, infc: &'a Interface)  -> &'a str {
    //     //let infc: Interface = self.as_interface();
    //     &infc.body
    // }
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