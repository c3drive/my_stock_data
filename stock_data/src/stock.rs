//! 株価データを取得するためのLambda関数プロジェクトです。
use std::fs;
use std::io::{Write, Read, BufWriter, BufReader, copy};
use lambda_runtime::{handler_fn, Context, Error};
use reqwest::StatusCode;
use stock_data::*;
//use serde_json::{json, Value};
use std::collections::HashMap;

use crate::get_stock_data::Interface;
mod get_stock_data;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let func = handler_fn(my_handler);
    //lambda_runtime::run(func).await?;
    my_handler().await;
    Ok(())
}


async fn my_handler() -> Result<(), Box<dyn std::error::Error>> {
    let param = "$NIKK";
    let url = format!("https://stockcharts.com/h-sc/ui?s={}", param);
    let chart = get_stock_data::GetStockChart { url_base: "https://stockcharts.com/h-sc/ui".to_string(), code: "$NIKK".to_string() };
    let encoded = chart.send().await;

    println!("size: {:?}", encoded);

    println!("call: {}", url);
    if let Ok(res) = reqwest::get(&url).await {
        match res.status() {
            StatusCode::OK => {
                let body = res.text().await?;
                //println!("response is \n{}", body);
                //file_write(body);

                let links = get_links(body, "https:".to_string())?;
                for link in links.iter() {
                    println!("chart: {}", link);
                }
            },
            StatusCode::NOT_FOUND => {
                println!("error: 目的のページがありませんでした。");
            },
            _ => {
                println!("error: その他のエラーが発生しました。");
            }
        }
    } else {
        println!("error: Webサーバーが見つかりませんでした。");
    }
    //wget --secure-protocol=auto "https://stockcharts.com/c-sc/sc?s=%24NIKK&p=D&b=5&g=0&i=0&r=1631528503869" --user-agent="Mozilla/5.0"
    Ok(())
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    // Ok(())
}


fn file_write(string: String) {
    {
        // write
        //let string = "Hello, file io!";
        let mut f = fs::File::create("test.txt").unwrap(); // open file, you can write to file.
        // "create" open as write only mode.
        f.write_all(string.as_bytes()).unwrap(); // byte-only

        // file is closed here.
    }
}



#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }
}
