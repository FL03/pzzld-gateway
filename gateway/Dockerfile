FROM rust as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as upgraded

RUN apt-get install -y \
    clang-11 \
    curl \
    wget

FROM upgraded as builder-base

ENV PATH="${PATH}:/go/bin"

RUN wget https://go.dev/dl/go1.19.linux-amd64.tar.gz && \
    tar xvf go1.19.linux-amd64.tar.gz

RUN rm go1.19.linux-amd64.tar.gz && \
    export PATH=$PATH:/go/bin

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release --workspace

FROM debian:buster-slim

ENV MODE="production" \
    SERVER_PORT=8080 \
    RUST_LOG="debug"

COPY Gateway.toml /config/Gateway.toml
COPY --from=builder /workspace/target/release/gateway /bin/gateway

EXPOSE ${SERVER_PORT}

CMD [ "gateway" ]