## This repo
```bash
PC(VS Code, Docker for Container1)
└── Container1(Docker for Container2, AWS SAM, Python etc..)
    └── Container2(Run functions locally and invoke)
```

## Common in the first time
When you clone for the first time,

### 1. aws condential files
put the aws condential files in this mount directory.

Example config.
```
[default]
region=ap-northeast-1
output=json
```
Example credentials.
```
[default]
aws_access_key_id = XXXXXXXXX
aws_secret_access_key = XXXXXXXXX
```
mount directory
```bash
workspace/.aws/{condential files}
```
### 2. workspace/samconfig.toml
You can create it later form `sam deploy --guided` without creatting now.

Example samconfig.toml.
```
version = 0.1
[default]
[default.deploy]
[default.deploy.parameters]
stack_name = "xxxxxxxxxxxxxxxxxxxxxx"
s3_bucket = "xxxxxxxxxxxxxxxxxxxxxx"
image_repository = "xxxxxxxxxxxxxxxx"
region = "ap-northeast-1"
confirm_changeset = true
capabilities = ["CAPABILITY_IAM", "CAPABILITY_NAMED_IAM", "CAPABILITY_AUTO_EXPAND"]
```
mount directory
```bash
workspace/samconfig.toml
```

### 3. env files
make enviroment files in this mount directory.

Example .env.(use dev: cargo run)
```
TEST=HelloEnv
AWS_S3_BUCKET=***********
```
Example env.json(use invoke: sam local invoke --env-vars env.json)
```
{
    "HelloRust": {
        "TEST": "HelloEnv!",
        "AWS_S3_BUCKET": "***********"
    },
    "StockRust": {
        "TEST": "HelloEnv!!",
        "AWS_S3_BUCKET": "***********"
    }
  }
```
Example template.yaml(use build&deploy)
```
Parameters:
  AwsS3Bucket:
    Type: String

Resources:
  StockRustFunction:
    Type: AWS::Serverless::Function
    Properties:
      Environment: 
        Variables:
          AWS_S3_BUCKET: !Ref AwsS3Bucket
```

Example samconfig.toml.
```
parameter_overrides = "AwsS3Bucket=my-work-project-bucket"
```
It does not have to be set samconfig.toml.
patern1. not set. you must `sam deploy --parameter-overrides AwsS3Bucket=**********`.
patern2. set. you must `sam deploy`.

mount directory
```bash
workspace/stock_data/.env
workspace/env.json
```


### 4. event files
make event files in this mount directory.

Example event.json.(use invoke: sam local invoke --e events/event.json)
```
{
    "ticker": "$NIKK"
}
```
mount directory
```bash
workspace/events/event.json
```

## Common in Container1
```bash
$ service docker start
```

## workspace in Container1
Run functions locally and invoke them
```bash
$ sam build
$ sam local invoke -e events/event.json --env-vars env.json StockRustFunction
```

To build and deploy
```bash
$ sam build
$ sam deploy --guided // if you do not use samconfig.toml.
$ sam deploy --parameter-overrides AwsS3Bucket=********** // if you do not use a parts of samconfig.toml.
$ sam deploy // if you use samconfig.toml.
```
## bin

Sample toml
```toml
[[bin]]
name = "stock"
path = "src/stock.rs"
```
sample Command
```bash
cargo run --bin stock 
cargo build --bin stock 
cargo build --bin stock --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/comment $(ARTIFACTS_DIR)/bootstrap
```

## cargo

```bash
$ cd stock_data
$ cargo run --bin stock
$ cargo test
$ cargo test --bin stock
$ cargo build
```
