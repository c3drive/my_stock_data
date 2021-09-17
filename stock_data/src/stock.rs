//! 株価データを取得するためのLambda関数プロジェクトです。
//use std::fs;
//use std::io::{Write, Read, BufWriter, BufReader, copy};
use lambda_runtime::{Error};
//use lambda_runtime::{handler_fn, Context, Error};
// use reqwest::StatusCode;
//use serde_json::{json, Value};
use stock_data::{Interface, InterfaceTrait, get_links};
use std::collections::HashMap;
mod get_stock_data;
use get_stock_data::GetStock;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let func = handler_fn(my_handler);
    //lambda_runtime::run(func).await?;
    my_handler().await.unwrap_or_else(|err| eprintln!("{}", err));
    //println!("{:?}", my_handler().await);
    Ok(())
}


async fn my_handler() -> Result<(), Box<dyn std::error::Error>> {

    // SetUp
    let mut chart = GetStock::new();

    // Request
    chart.send_request().await?;
    let body = chart.get_data();

    // GetUrl
    let link = get_url(body);

    //let links = get_links(body, "https:".to_string())?;
    //for link in links.iter() {
    //    println!("chart: {}", link);
    //}
    // 取得したurlで画像dL（部品か）
    // 保存
    //wget --secure-protocol=auto "https://stockcharts.com/c-sc/sc?s=%24NIKK&p=D&b=5&g=0&i=0&r=1631528503869" --user-agent="Mozilla/5.0"
    Ok(())
}
fn get_url(body: String) -> String {
    // チャート画像抜き出し（1つしかない想定なので、一番最初のURLを使う）
    let links = get_links(&body, "https:".to_string());
    let url = &links[0];
    return String::from(url);
    // for link in links.iter() {
    //     chart_url = link;
    //     println!("chart: {}", link);
    // }
    //return url;
}

// fn file_write(string: String) {
//     {
//         // write
//         //let string = "Hello, file io!";
//         let mut f = fs::File::create("test.txt").unwrap(); // open file, you can write to file.
//         // "create" open as write only mode.
//         f.write_all(string.as_bytes()).unwrap(); // byte-only

//         // file is closed here.
//     }
// }



#[cfg(test)]
mod tests {
     //use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }
}
