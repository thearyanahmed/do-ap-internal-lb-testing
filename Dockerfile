FROM rustlang/rust:nightly-slim

RUN apt-get update && apt-get install -y \
    iputils-ping \
    procps \
    top \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV RUST_LOG=info

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/test0"]
