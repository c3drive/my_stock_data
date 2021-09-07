FROM rust:latest

#COPY app.py s3example.py requirements.txt ./

#RUN python3.9 -m pip install -r requirements.txt -t .

# Command can be overwritten by providing a different command in the template directly.
#CMD ["app.lambda_handler"]

RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl