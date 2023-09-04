use std::{fmt::Debug, str::FromStr};

pub fn read_env<F, E>(key: impl ToString) -> F
where
    F: FromStr<Err = E>,
    E: Debug,
{
    let key = key.to_string();

    dotenv::var(&key)
        .expect(&format!("failed to read {key}"))
        .parse()
        .expect(&format!("failed to parse {key}"))
}

pub fn read_optional_env<F, E>(key: impl ToString) -> Option<F>
where
    F: FromStr<Err = E>,
    E: Debug,
{
    let key = key.to_string();

    dotenv::var(&key)
        .ok()
        .map(|value| value.parse().expect(&format!("failed to parse {key}")))
}
