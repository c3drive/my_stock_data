use std::collections::HashMap;
use async_trait::async_trait;
use reqwest::StatusCode;
use stock_data::*;
use thiserror::Error;

pub struct GetStockChart {
    pub url_base: String,
    pub code: String,
    //pub res: Response,
}
// struct PineApple {
//     url_base: String,
//     code: String,
// }

// 抽象化（AppleTraitはget_sizeを実装する必要がある）
#[async_trait]
pub trait Interface {
    //type Item;
    type Error;
    async fn send(&self)-> Result<(), Self::Error>;
    fn on_parse(&self) -> String;
    async fn get_data(&self) -> String;
}
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Record not found")]
    NotFound,
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Failed to get connection")]
    ConncectionPoolError(#[from] reqwest::Error),

    // #[error("Failed SQL execution")]
    // SQLiteError(#[from] rusqlite::Error),
}
//impl std::error::Error for ApiError {}
// impl fmt::Display for ApiError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             ApiError::NotFound => f.write_str("NotFound"),
//             ApiError::InternalServerError => f.write_str("InternalServerError"),
//         }
//     }
// }
// impl Error for ApiError {
//     fn description(&self) -> &str {
//         match *self {
//             ApiError::NotFound => "Record not found",
//             ApiError::InternalServerError => "Internal server error",
//         }
//     }
// }
#[async_trait]
impl Interface for GetStockChart {
    //type Item = HashMap<String, String>;
    //type Error2 = Box<dyn std::error::Error + 'static>;
    //type Error2 = Box<dyn ApiError + 'static>;
    type Error = ApiError;
    
    async fn send(&self) -> Result<(), Self::Error> {

        make_log("send", "start");
        let url = format!("{}?s={}", self.url_base, self.code);

        make_log("send", "reqwest::get start");
        println!("url: {}", url);
        let resp = reqwest::get(&url).await?;
        make_log("send", "reqwest::get end");
        println!("Response: {:?}", resp);

        // if let Ok(res) = reqwest::get(&url).await {
        //     match res.status() {
        //         StatusCode::OK => {
        //            let body = res.text().await?;
        //             println!("response is \n{}", body);

        //             let links = get_links(body, "https:".to_string())?;
        //             for link in links.iter() {
        //                 println!("chart: {}", link);
        //             }
        //         },
        //         StatusCode::NOT_FOUND => {
        //             println!("error: 目的のページがありませんでした。");
        //         },
        //         _ => {
        //             println!("error: その他のエラーが発生しました。");
        //         }
        //     }
        // } else {
        //     println!("error: Webサーバーが見つかりませんでした。");
        // }
          Ok(())
    }
    fn on_parse(&self) -> String {
        "GetStockChart parse".to_string()
    }
    async fn get_data(&self) -> String {
        "GetStockChart parse".to_string()
    }
}
// impl Interface for PineApple {
//     fn send(&self)  {
//         let url = format!("{}?q={}", self.url_base, self.code);
//         println!("{}", url);
//     }
//     fn on_parse(&self) -> String {
//         "PineApple parse".to_string()
//     }
//     fn get_data(&self) -> String {
//         "PineApple parse".to_string()
//     }
// }
#[tokio::main]
async fn main() {

    println!("start");
    let chart = GetStockChart { url_base: "https://stockcharts.com/h-sc/ui".to_string(), code: "$NIKK".to_string() };
    let encoded = chart.send().await;

    println!("size: {:?}", encoded);
    // let encoded = match encoded {
    //     Ok(file) => {
    //         println!("{:#?}", file);
    //     },
    //     Err(error) => {
    //         // ファイルを開く際に問題がありました
    //         panic!("There was a problem opening the file: {:?}", error);
    //     },
    // };
    //et pine_apple = PineApple { url_base: "http://google.com/".to_string(), code: "2".to_string() };

    let data = chart.get_data().await;
    println!("size1: {}", data);
    println!("size2: {}", chart.get_data().await);

}