FROM rustlang/rust:nightly-slim

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/test0"]
