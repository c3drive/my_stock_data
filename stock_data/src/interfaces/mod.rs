pub mod get_stockcharts;


//####################################################
// ↓↓↓↓↓↓↓　Interface Base class ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
//####################################################
use async_trait::{async_trait};
use reqwest::StatusCode;
use std::collections::HashMap;
use thiserror::Error;

use stock_data::make_log;

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
}
// IF実装する際のトレイト
#[async_trait]
pub trait Interface: Sync + Send {

    // コンストラクタ
    fn new() -> Self ;

    // デフォルトは何もしない。パラメータがあれば各IFで実装
    fn add_param(&mut self, _values: Vec<String>) {
        // Default
        //let keys = vec![String::from("key1"), String::from("key2"),..];
        //let params: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
        //self.url = String::from(Url::parse_with_params(&self.url, params).unwrap());
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