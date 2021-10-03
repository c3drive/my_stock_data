use async_trait::{async_trait};
use bytes::Bytes;
use egg_mode::{KeyPair, RateLimit, Token, error::Error, media::{media_types, upload_media}, tweet::{DraftTweet, Tweet}};
use reqwest::Response;
//use reqwest::Response;
use stock_data::*;
use std::collections::HashMap;
use url::Url;
use crate::{config::config::CONFIG, interfaces::{ApiError, Interface, send}};

pub struct PostTweetIF {
    token: Token,
    draft: DraftTweet,
    ratelimit: Option<RateLimit>,
    content: HashMap<String, String>,
    verbose: bool,
}

pub struct TweetResult {
    ratelimit: RateLimit,
    tweet: Tweet,
}
impl PostTweetIF {

    // コンストラクタ
    pub fn new(text: &str) -> PostTweetIF {  
        let token = Token::Access {
            consumer: KeyPair::new(
                CONFIG.consumer_key.to_string(),
                CONFIG.consumer_secret.to_string()
            ),
            access: KeyPair::new(
                CONFIG.access_token.to_string(),
                CONFIG.access_token_secret.to_string()
            )
        };      
        PostTweetIF {
            token: token,
            draft: DraftTweet::new(String::from(text)),
            ratelimit: None,
            content: HashMap::new(),
            verbose: true,
        }
    }
    
    // 画像アップロード
    pub async fn add_image(&mut self, byte: Bytes) -> Result<(), Error> {
        let handle = upload_media(&byte, &media_types::image_png(), &self.token).await?;
        &self.draft.add_media(handle.id);
        Ok(())
    }

    // リクエスト送信
    pub async fn send_request(&mut self) -> Result<TweetResult, Error> {
        make_log("[INFO]", "send_request", "start");

        let response = self.draft.send(&self.token).await?;
        let result = self.on_parse(response).await;

        make_log("[INFO]", "send_request", "end");
        return Ok(result);
    }

    // レスポンスパース
    async fn on_parse(&mut self, response: egg_mode::Response<Tweet>) -> TweetResult {
        make_log("[INFO]", "on_parse", "start");

        // limitが-1以外だったら問題
        make_log("[INFO]", "on_parse", "rate_limit_status start");
        let ratelimit = response.rate_limit_status;
        //println!("{:?}", ratelimit);

        // 問題がなければ取得
        make_log("[INFO]", "on_parse", "rate_limit_status start");
        let tweet = response.response;
        //println!("{:?}", tweet);

        // url取得
        let id = &tweet.id;
        let screen_name = &tweet.user.as_ref().unwrap().screen_name;
        let tweet_url = String::from("https://twitter.com/") + screen_name.as_str() + "/status/" + id.to_string().as_str();

        // 結果格納
        &self.content.insert(String::from("created_at"), (&tweet.created_at).to_string());
        &self.content.insert(String::from("text"), (&tweet.text).to_string());
        &self.content.insert(String::from("url"), tweet_url);

        println!();
        if self.verbose {
            println!("created_at: {}", &self.content["created_at"]);
            println!("text:       {}", &self.content["text"]);
            println!("url:        {}", &self.content["url"]);
            println!();
        }

        // 未加工データを利用する場合
        let result = TweetResult {
            ratelimit: ratelimit,
            tweet: tweet,
        };

        make_log("[INFO]", "on_parse", "end");
        return result;
    }

    // 返却
    pub fn get_content(&self) -> HashMap<String, String> {
        self.content.clone()
    }
    
}