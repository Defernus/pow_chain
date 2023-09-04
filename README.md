# Proof of work challenge-response server

Simple challenge-response server in rust with proof of work.

**NOTE:** this project is just for learning purposes, it is not intended to be used in production.

## PoW algorithm choice motivation

- Well-tested and Secure: SHA-256, as part of the SHA-2 family, has been extensively reviewed and tested by cryptographic experts worldwide. This means that vulnerabilities and weaknesses would have been well-identified by now, making it a robust choice for PoW.

- Wide Acceptance: Bitcoin, being the first and one of the most prominent cryptocurrencies, uses SHA-256 as its PoW algorithm. This wide acceptance in a high-stakes environment speaks to its reliability.

- Adjustable Difficulty: One of the key features of Bitcoin's PoW is the ability to adjust the difficulty of the puzzle. By changing the number of leading zeros required in the hash, we can make the problem more or less difficult, allowing us to calibrate the challenge based on our needs.

- Asymmetric Nature: The SHA-256 PoW requires significant computational effort to solve (find a hash with a specific number of leading zeros), but once a solution is found, verifying it is trivial. This asymmetry ensures that challengers spend computational resources while the server can quickly validate their efforts.

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

- Run node:
  ```bash
  cargo run --bin pow_node
  ```
- Run client:
  ```bash
  cargo run --bin pow_client
  ```

## Examples

Run examples:

```
cargo run --example <example name>
```

For example:

```
cargo run --example mining
```

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT.
