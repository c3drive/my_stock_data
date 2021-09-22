/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

 use aws_config::meta::region::RegionProviderChain;
 use aws_sdk_s3::{ByteStream, Client, Error, Region, PKG_VERSION};
 
 use std::path::Path;
 use std::process;

 use async_trait::{async_trait};
 use stock_data::*;
 //use crate::interfaces::{ApiError, Interface, send};
 
 struct ManageS3IF {
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
    async fn new(region: Option<String>, bucket: String, key: String) -> ManageS3IF {
        let region_provider = RegionProviderChain::first_try(region.map(Region::new))
            .or_default_provider()
            .or_else(Region::new("ap-northeast-1"));

        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);

        ManageS3IF {
            //region_provider: region_provider,
            //region: region,
            client: client,
            bucket: bucket,
            key: key,
            verbose: true,
        }
    }
    // async fn make_client(&self) -> Client {

    //     // println!();
    
    //     // if opt.verbose {
    //     //     println!("S3 client version: {}", PKG_VERSION);
    //     //     println!("Region:            {}", shared_config.region().unwrap());
    //     //     println!("Bucket:            {}", &opt.bucket);
    //     //     println!("Key:               {}", &opt.key);
    //     //     println!();
    //     // }
    //     return client;
    // }
    async fn list(&self) -> Result<(), Error> {
        //let client = self.make_client().await;
 
        // バケット一覧取得
        let resp = self.client.list_buckets().send().await?;
    
        for bucket in resp.buckets.unwrap_or_default() {
            println!("bucket: {:?}", bucket.name.as_deref().unwrap_or_default())
        }
        Ok(())
    }
}
 /// Lists your buckets and uploads a file to a bucket.
 /// # Arguments
 ///
 /// * `-b BUCKET` - The bucket to which the file is uploaded.
 /// * `-k KEY` - The name of the file to upload to the bucket.
 /// * `[-r REGION]` - The Region in which the client is created.
 ///    If not supplied, uses the value of the **AWS_REGION** environment variable.
 ///    If the environment variable is not set, defaults to **us-west-2**.
 /// * `[-v]` - Whether to display additional information.
 #[tokio::main]
 async fn main() -> Result<(), Error> {
    let opt = ManageS3IF::new(
        None, // デフォルトを利用する
        String::from("my-work-project-bucket"),
        String::from("test.png"),
    ).await;
 
    opt.list().await?;
 
 
     // アップロードデータ
    //  let body = ByteStream::from_path(Path::new("test.png")).await;
 
    // // アップロード
    //  match body {
    //      Ok(b) => {
    //          let resp = client
    //              .put_object()
    //              .bucket(&opt.bucket)
    //              .key(&opt.key)
    //              .body(b)
    //              .send()
    //              .await?;
 
    //          println!("Upload success. Version: {:?}", resp.version_id);
 
    //          let resp = client.get_object().bucket(opt.bucket).key(opt.key).send().await?;
    //          let data = resp.body.collect().await;
    //          println!("data: {:?}", data.unwrap().into_bytes());
    //      }
    //      Err(e) => {
    //          println!("Got an error DOING SOMETHING:");
    //          println!("{}", e);
    //          process::exit(1);
    //      }
    //  }
 
     Ok(())
 }