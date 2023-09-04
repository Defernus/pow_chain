FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install --path ./packages/node

CMD ["pow_chain_node"]
