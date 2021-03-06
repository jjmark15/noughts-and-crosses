FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin server
WORKDIR ./server
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./crates ./crates
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/server*
RUN cargo build --release


FROM alpine:latest

LABEL org.opencontainers.image.source=https://github.com/jjmark15/noughts-and-crosses

ARG APP=/usr/src/app

EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/server/target/x86_64-unknown-linux-musl/release/server ${APP}/server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./server"]
