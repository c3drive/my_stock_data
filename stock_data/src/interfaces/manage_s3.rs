/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

 use aws_config::meta::region::RegionProviderChain;
 use aws_sdk_s3::{ByteStream, Client, Error, Region, PKG_VERSION};
 
 use std::path::Path;
 use std::process;

 use crate::config::config::CONFIG;
 use stock_data::*;
 //use crate::interfaces::{ApiError, Interface, send};
 
 pub struct ManageS3IF {
    /// The AWS Client.
    client: Client,

    /// The AWS Config.
    //shared_config: aws_types::config::Config,

     /// The AWS Region.
    //region_provider: RegionProviderChain,
    //region: Option<String>,
 
     /// The name of the bucket.
     bucket: String,
 
     /// The name of the object in the bucket.
     key: String,
 
     /// Whether to display additional information.
     verbose: bool,
 }

//impl Interface for ManageS3IF {
impl ManageS3IF {

    // コンストラクタ
    pub async fn new(region: Option<String>, bucket: String, key: String) -> ManageS3IF {
        let region_provider = RegionProviderChain::first_try(region.map(Region::new))
            .or_default_provider()
            .or_else(Region::new("ap-northeast-1"));

        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);

        let manage = ManageS3IF {
            //region_provider: region_provider,
            //region: region,
            client: client,
            bucket: bucket,
            key: key,
            verbose: true,
        };

        println!();
    
        if manage.verbose {
            println!("S3 client version: {}", PKG_VERSION);
            println!("Region:            {}", shared_config.region().unwrap());
            println!("Bucket:            {}", &manage.bucket);
            println!("Key:               {}", &manage.key);
            println!();
        }
        return manage;
    }

    /// バケット一覧表示
    pub async fn list(&self) -> Result<(), Error> {
        make_log("[INFO]", "list", "start");
 
        // バケット一覧取得
        let resp = self.client.list_buckets().send().await?;
    
        for bucket in resp.buckets.unwrap_or_default() {
            println!("bucket: {:?}", bucket.name.as_deref().unwrap_or_default())
        }
        Ok(())
    }

    /// アップロード
    pub async fn push(&self, path: &str) -> Result<(), Error> {
        make_log("[INFO]", "push", "start");

        // アップロードデータ
        make_log("[INFO]", "push", "get ByteStream");
        let body = ByteStream::from_path(Path::new(path)).await;
    
        // アップロード
        make_log("[INFO]", "put", "send start");
        match body {
            Ok(b) => {
                let resp = self.client
                    .put_object()
                    .bucket(&self.bucket)
                    .key(&self.key)
                    .body(b)
                    .send()
                    .await?;
    
                println!("Upload success. Version: {:?}", resp.version_id);
    
                let resp = self.client.get_object().bucket(&self.bucket).key(&self.key).send().await?;
                let data = resp.body.collect().await;
                println!("data: {:?}", data.unwrap().into_bytes());
            }
            Err(e) => {
                println!("Got an error DOING SOMETHING:");
                println!("{}", e);
                process::exit(1);
            }
        }
        Ok(())
    }
}
//  #[tokio::main]
//  async fn main() -> Result<(), Error> {
//     let s3 = ManageS3IF::new(
//         None, // デフォルトを利用する
//         String::from("my-work-project-bucket"),
//         String::from("tmp.png"),
//     ).await;
 
//     s3.list().await?;
//     s3.push("tmp.png").await?;
 
//     Ok(())
//  }

#[cfg(test)]
mod tests {
     use super::*;

    #[tokio::test]
    async fn it_env() {
        let s3_object = String::from("stock_data/");
    
        // SetUp
        let s3 = ManageS3IF::new(
            None, // デフォルトを利用する
            CONFIG.aws_s3_bucket.to_string(),
            s3_object,
        ).await;
        assert_eq!(s3.bucket, String::from("my-work-project-bucket"));
        //assert!(true, "always true");
    }
}