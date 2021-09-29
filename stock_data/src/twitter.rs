use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    ticker: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    result: String,
    imgurl: String,
    filename: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    //func().await?;

    Ok(())
}
async fn func(event: CustomEvent, _: Context) -> Result<CustomOutput, Error> {
    // イベントを受け取る（S3のプレフィックストリガー、要バケット名、キー名）
    // S3から画像取得
    // twitter投稿
        Ok(CustomOutput {
            result: String::from(format!("Ok, {}!", "ttt")),
            imgurl: String::from(format!("{}", "ttt")),
            filename: String::from(format!("{}", "ttt")),
        })
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn is_correct_func_response() -> Result<(), Error> {
        let ticker = "$NIKK";
        let event = CustomEvent{ticker: String::from(ticker),};

        // 実行
        let response = func(event, Context::default()).await?;

        // 結果
        let json = CustomOutput {
            result: String::from(format!("Ok, {}!", "ttt")),
            imgurl: String::from(format!("{}", "ttt")),
            filename: String::from(format!("{}", "ttt")),
        };
        // アサーション
        assert_eq!(response.result, json.result);
        assert_eq!(response.filename, json.filename);

        Ok(())
    }
}