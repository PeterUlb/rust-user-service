FROM rust:1.46

WORKDIR /usr/src/user_service
COPY . .

RUN cargo build --release

CMD ["target/release/user_service"]

#run with e.g. docker run --env APP_DATABASE.URL=abc imageid