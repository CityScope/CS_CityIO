FROM rust:1.51 as builder

RUN USER=root cargo new --bin cityio
WORKDIR ./cityio
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./ 

RUN rm ./target/release/deps/cityio*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /cityio/target/release/cityio ${APP}/cityio

ENV REDIS_ADDR=127.0.0.1
ENV REDIS_PORT=6379

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./cityio"]

