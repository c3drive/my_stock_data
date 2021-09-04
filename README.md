## This repo
```bash
PC(VS Code, Docker for Container1)
└── Container1(Docker for Container2, AWS SAM, Python etc..)
    └── Container2(Run functions locally and invoke)
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
$ sam deploy --guided
```
General information about this SAM project can be found in the [`README.md`](./lambda-python3.9/README.md) file in lambda-python3.9 folder.