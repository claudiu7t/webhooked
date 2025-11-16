FROM rust:1.91.1
WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
CMD ["cargo", "run"]
