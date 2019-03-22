FROM rust:1.33

WORKDIR /usr/src/rust-greetings
COPY src/ ./src/
COPY Cargo.* ./

RUN cargo install --path .

CMD ["rust-greetings"]
