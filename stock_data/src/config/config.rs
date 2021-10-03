use serde::Deserialize;
use config::ConfigError;
use dotenv::dotenv;
use once_cell::sync::Lazy;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub test: String,
    pub aws_s3_bucket: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub bearer_token: String,
    pub access_token: String,
    pub access_token_secret: String,
}
// 初期化（定数へセット）
pub static CONFIG: Lazy<Config> = Lazy::new(|| 
    {
        dotenv().ok();
        Config::from_env().unwrap()
    }
);

// 環境変数を読み込む
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

// fn main() {
//     dotenv().ok();
//     println!("{:?}", Config::from_env());
//     println!("{:?}", CONFIG.test);
//     println!("{:?}", CONFIG.aws_s3_bucket);
// }
mod tests {
    use super::*;

    #[test]
    fn get_env() {
       assert_eq!(CONFIG.test, "HelloEnv");
   }
}