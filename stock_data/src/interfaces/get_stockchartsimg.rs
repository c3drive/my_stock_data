use async_trait::{async_trait};
use reqwest::Response;
use stock_data::*;
use std::collections::HashMap;
use bytes::Bytes;
use crate::interfaces::{ApiError, InterfaceDirect, send};

pub struct GetStockChartsImgIF {
    url: String,
    _body: String,
    content: HashMap<String, Bytes>,
}

#[async_trait]
impl InterfaceDirect for GetStockChartsImgIF {

    // コンストラクタ
    fn new(url: &String) -> GetStockChartsImgIF {        
        GetStockChartsImgIF {
            url: String::from(url),
            _body: String::new(),
            content: HashMap::new(),
        }
    }

    // リクエスト送信
    async fn send_request(&mut self) -> Result<(), ApiError> {
        make_log("[INFO]", "send_request", "start");

        make_log("[INFO]", "send_request", "send start");
        let response = send(&self.url).await?;
        self.on_parse(response).await;

        make_log("[INFO]", "send_request", "end");
        return Ok(());
    }

    // レスポンスパース
    async fn on_parse(&mut self, response: Response) -> Result<(), ApiError> {
        make_log("[INFO]", "on_parse", "start");

        // パース
        make_log("[INFO]", "on_parse", "reqwest::text start");
        // let text = response.text().await;
        // let httpxml = match text {
        //     Ok(httpxml) => httpxml,
        //     Err(e) => {
        //         return Err(ApiError::ResponseBody(e));
        //     }
        // };
        // let body = httpxml;

        make_log("[INFO]", "on_parse", "reqwest::bytes start");
        let res_bytes = response.bytes().await;
        let bytes = match res_bytes {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(ApiError::ResponseBody(e));
            }
        };

        // 結果格納
        //&self.body.push_str(&body);

        //&self.content.insert(String::from("body"), bytes);
        &self.content.insert(String::from("bytes"), bytes);

        make_log("[INFO]", "on_parse", "end");
        return Ok(());
    }

    // 返却
    fn get_content(&self) -> HashMap<String, Bytes> {
        self.content.clone()
    }
    
}