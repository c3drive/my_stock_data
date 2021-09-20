use async_trait::{async_trait};
use stock_data::*;
use std::collections::HashMap;
use url::Url;
use crate::interfaces::{ApiError, Interface, send};

pub struct GetStockChartsIF {
    url: String,
    body: String,
    content: HashMap<String, String>,
}

#[async_trait]
impl Interface for GetStockChartsIF {

    // コンストラクタ
    fn new() -> GetStockChartsIF {        
        GetStockChartsIF {
            url: String::from("https://stockcharts.com/h-sc/ui"),
            body: String::new(),
            content: HashMap::new(),
        }
    }
    
    // パラメータセット（オーバーライド）
    fn add_param(&mut self, values: Vec<String>) {
        let keys = vec![String::from("q"),];
        let params: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
        self.url = String::from(Url::parse_with_params(&self.url, params).unwrap());
    }

    // リクエスト送信
    async fn send_request(&mut self) -> Result<(), ApiError> {
        make_log("[INFO]", "send_request", "start");

        make_log("[INFO]", "send_request", "send start");
        let httpxml = send(&self.url).await?;
        self.on_parse(httpxml);

        make_log("[INFO]", "send_request", "end");
        return Ok(());
    }

    // レスポンスパース
    fn on_parse(&mut self, httpxml: String) {
        make_log("[INFO]", "on_parse", "start");

        // パースの必要なし
        let body = httpxml;

        // チャート画像URL抜き出し（1つしかない想定なので、一番最初のURLを使う）
        let links = get_links(&body, "https:".to_string());
        let url = String::from(&links[0]);
    
        // 結果格納
        &self.body.push_str(&body);

        &self.content.insert(String::from("body"), body);
        &self.content.insert(String::from("url"), url);

        make_log("[INFO]", "on_parse", "end");
    }

    // 返却
    fn get_content(&self) -> HashMap<String, String> {
        self.content.clone()
    }
    
}