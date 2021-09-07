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
### 2. lambda-python3.9/samconfig.toml
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
workspace/lambda-python3.9/samconfig.toml
```

## Common in Container1
```bash
$ service docker start
```

## lambda-python3.9 in Container1
Run functions locally and invoke them
```bash
$ cd lambda-python3.9
$ sam build
$ sam local invoke
```

Use the `sam local start-api` to run the API locally on port 3000.
```bash
$ cd lambda-python3.9
$ sam build
$ sam local start-api
```
```
$ curl http://127.0.0.1:3000/hello
{"message": "hello world"}
```
To build and deploy
```bash
$ cd lambda-python3.9
$ sam build
$ sam deploy --guided // if you do not use samconfig.toml.
$ sam deploy // if you use samconfig.toml.
```


## Unit tests

```bash
$ cd lambda-python3.9
$ pip install pytest pytest-mock --user
$ python -m pytest tests/ -v
```
General information about this SAM project can be found in the [`README.md`](./lambda-python3.9/README.md) file in lambda-python3.9 folder.