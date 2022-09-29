FROM rust:1.64 AS build

WORKDIR /usr/src/buzzer
COPY . .

RUN apt update \
    && apt install -y protobuf-compiler \
    && cargo install --path .

CMD ["buzzer"]
