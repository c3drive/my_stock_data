use async_trait::{async_trait};
use reqwest::StatusCode;
use stock_data::*;
use thiserror::Error;

pub struct GetStockChart {
    pub url_base: String,
    pub code: String
}

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
    //type GetStockChartResponse;
    //type Error2 = Box<dyn std::error::Error + 'static>;
    //type Error2 = Box<dyn ApiError + 'static>;
    type Error = ApiError;
    
    async fn send(&self) -> Result<GetStockChartResponse, Self::Error> {

        make_log("send", "start");
        let url = format!("{}?s={}", self.url_base, self.code);

        make_log("send", "reqwest::get start");
        println!("url: {}", url);
        //let res = reqwest::get(&url).await?;
        //make_log("send", "reqwest::get end");
        //println!("Response: {:?}", res);

        if let Ok(res) = reqwest::get(&url).await {
            // Check if status is within 200-299.
            if res.status().is_success() {
                let body = res.text().await?;

                // チャート画像抜き出し（1つしかない想定なので、一番最初のURLを使う）
                let links = get_links(&body, "https:".to_string())?;
                let url = &links[0];
                //for link in links.iter() {
                    //chart_url = link;
                //     println!("chart: {}", link);
                //}
                let ret = GetStockChartResponse::new(
                    String::from("success"),
                    body,
                    url,
                );
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
#[tokio::main]
async fn main() {
    println!(r#"start"#);
    let chart = GetStockChart { url_base: "https://stockcharts.com/h-sc/ui".to_string(), code: "$NIKK".to_string() };
    if let Ok(encoded) = chart.send().await {
        println!("size: {:?}", encoded);
        println!("size: {:?}", encoded.get_data());
        println!("size: {:?}", encoded.get_url());
    }else {
        println!("err");
    };
}