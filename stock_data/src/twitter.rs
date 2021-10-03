mod interfaces;
mod config;

use bytes::Bytes;
use chrono::DateTime;
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{BufReader, Read}};

use interfaces::manage_s3::ManageS3IF;
use interfaces::post_tweet::PostTweetIF;

#[derive(Debug, Clone, Serialize, Deserialize)]
//#[serde(rename_all = "PascalCase")]
struct S3PutEvent {
    // 大文字始まりのjsonを小文字（records）で探してしまうので大文字（Records）明示
    // #[serde(rename_all = "PascalCase")]でもOK
    #[serde(rename = "Records")]
    Records: Vec<Record>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    eventTime: String,
    s3: S3Data,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct S3Data {
    bucket: Bucket,
    object: Object,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Bucket {
    name: String,
    arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Object {
    key: String,
    size: usize,

    #[serde(rename = "eTag")]
    etag: String,

    sequencer: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    result: String,
    tweet_time: String,
    tweet_text: String,
    tweet_url: String,
    key: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;

    // let file = File::open("../events/s3.json").unwrap();
    // let reader = BufReader::new(file);

    // let event: S3PutEvent = serde_json::from_reader(reader).unwrap();
    // println!("deserialized = {:?}", event);

    // let ret = func(event, Context::default()).await?;
    // println!("{:?}", ret);

    Ok(())
}
async fn func(event: S3PutEvent, _: Context) -> Result<CustomOutput, Error> {
    // イベントを受け取る
    let time = &event.Records[0].eventTime;
    let s3_data = &event.Records[0].s3;

    // S3から画像取得
    let byte = get_image(s3_data).await?;

    // tweet
    let dt = DateTime::parse_from_rfc3339(time)?;
    let datetime = dt.format("%Y年%m月%d日 %H時%M分").to_string();
    let text = String::from("日経平均株価　INDEXNIKKEI: NI225\n") + &datetime + "時点(©StockCharts)"; 
    let bodys = tweet(text, byte).await?;
    let url = &bodys["url"];

    Ok(CustomOutput {
        result: String::from(format!("Ok!")),
        tweet_time: String::from(format!("{}", &bodys["created_at"])),
        tweet_text: String::from(format!("{}", &bodys["text"])),
        tweet_url: String::from(format!("{}", &bodys["url"])),
        key: String::from(format!("{}", s3_data.object.key)),
    })
}
/// S3からダウンロード
async fn get_image(s3_data: &S3Data) -> Result<Bytes, Error> {
    // SetUp
    let s3 = ManageS3IF::new(
        None, // デフォルトを利用する
        s3_data.bucket.name.to_string(),
        s3_data.object.key.to_string(),
    ).await;
 
    // Request
    let byte_file = s3.get().await?;

    Ok(byte_file)
}

/// tweet
async fn tweet(text: String, byte: Bytes) -> Result<HashMap<String, String>, Error> {
    // SetUp
    let mut api = PostTweetIF::new(&text);

    // 画像アップロード
    api.add_image(byte).await?;
 
    // Request
    api.send_request().await?;

    // Result
    let bodys = api.get_content();

    Ok(bodys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn is_correct_func_response() -> Result<(), Error> {
        // テストデータ取得
        let file = File::open("../events/s3.json").unwrap();
        let reader = BufReader::new(file);    
        let event: S3PutEvent = serde_json::from_reader(reader).unwrap();

        // 実行
        let response = func(event, Context::default()).await?;

        // 結果
        let json = CustomOutput {
            result:     String::from(format!("Ok!")),
            tweet_time: String::from(format!("{}", "None")), // テスト不可
            tweet_text: String::from(format!("{}", "None")), // テスト不可
            tweet_url:  String::from(format!("{}", "None")), // テスト不可
            key:        String::from(format!("{}", "stock_data/$NIKK/$NIKK_20211002.png")),
        };
        // アサーション
        assert_eq!(response.result, json.result);
        assert_eq!(response.key, json.key);

        Ok(())
    }
}