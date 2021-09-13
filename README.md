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
put the aws condential files in this app directory.
You can create it later form `sam deploy --guided` without creatting now.

Example samconfig.toml.
```
version = 0.1
[default]
[default.deploy]
[default.deploy.parameters]
stack_name = "HelloWorld-Python"
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

## Common in Container1
```bash
$ service docker start
```

## workspace in Container1
Run functions locally and invoke them
```bash
$ sam build
$ sam local invoke
```

Use the `sam local start-api` to run the API locally on port 3000.
```bash
$ sam build
$ sam local start-api
```
```
$ curl http://127.0.0.1:3000/hello
{"message": "hello world"}
```
To build and deploy
```bash
$ sam build
$ sam deploy --guided // if you do not use samconfig.toml.
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

## Unit tests

```bash
$ cd stock_data
$ cargo test -- bin stock
```
