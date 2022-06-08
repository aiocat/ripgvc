FROM rust:1.60 as build

RUN USER=root cargo new --bin ripgvc
WORKDIR /ripgvc

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/ripgvc*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /ripgvc/target/release/ripgvc .

CMD ["./ripgvc"]