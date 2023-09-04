FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install --path ./packages/client

CMD ["pow_chain_client"]
