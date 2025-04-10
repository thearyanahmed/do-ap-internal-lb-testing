FROM rustlang/rust:nightly-slim

WORKDIR /app

ENV RUST_LOG=info

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/test0"]
