# Prof of work blockchain in rust

Simple blockchain implementation in rust with proof of work.

**NOTE:** this project is just for learning purposes, it is not intended to be used in production.

## Prerequisites

- install [rust](https://www.rust-lang.org/tools/install)
- install [cargo make](https://docs.rs/crate/cargo-make/latest)
- update .env file with your values (use .env.example as template)

## Tests

Rul all tests with:

```bash
cargo make test
```

## Run in docker

Run one node and one client:

```bash
docker-compose -f docker-compose.example.yml up --build --force-recreate --remove-orphans
```

## Run locally

Run node:

```bash
cargo run --bin pow_chain_node
```

Run client:

```bash
cargo run --bin pow_chain_client
```

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT.
