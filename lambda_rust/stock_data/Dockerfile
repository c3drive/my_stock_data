#FROM public.ecr.aws/bitnami/golang:1.16 as build-image
FROM rust:latest as build-image

#WORKDIR /go/src
WORKDIR /rust/stock_data
#COPY go.mod go.sum main.go ./
COPY src/ /rust/stock_data/src/
COPY Cargo.toml Cargo.lock /rust/stock_data/

#RUN go build -o ../bin
RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

#FROM public.ecr.aws/lambda/go:1
FROM public.ecr.aws/lambda/provided:al2

# 実行ファイルを起動するようにするため、ファイル名を "bootstrap" に変更する
	# Current Artifacts Directory : /workspace/stock_data/.aws-sam/build/HelloRustFunction
#COPY --from=build-image /go/bin/ /var/task/
COPY  --from=build-image /rust/stock_data/target/x86_64-unknown-linux-musl/release/hello ${LAMBDA_RUNTIME_DIR}/bootstrap

# カスタムランタイム同様ハンドラ名は利用しないため、適当な文字列を指定する。
CMD [ "lambda-handler" ]