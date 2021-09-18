use async_trait::{async_trait};
use stock_data::*;
use std::collections::HashMap;
use url::Url;


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
    
    // オーバーライド
    fn add_param(&mut self, params: HashMap<&str, &str>) {
        self.url = String::from(Url::parse_with_params(&self.url, params).unwrap());
    }

    async fn send_request(&mut self) -> Result<(), ApiError> {
        make_log("[INFO]", "send_request", "start");

        make_log("[INFO]", "send_request", "send start");
        let httpxml = send(&self.url).await?;
        self.on_parse(httpxml);

        make_log("[INFO]", "send_request", "end");
        return Ok(());
    }

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

    fn get_content(&self) -> HashMap<String, String> {
        self.content.clone()
    }
    
}