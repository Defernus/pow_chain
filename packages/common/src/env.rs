use std::{fmt::Debug, str::FromStr};

pub fn read_env<F, E>(key: impl ToString) -> F
where
    F: FromStr<Err = E>,
    E: Debug,
{
    let key = key.to_string();

    dotenvy::var(&key)
        .unwrap_or_else(|_| panic!("failed to read {key}"))
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse {key}"))
}

pub fn read_optional_env<F, E>(key: impl ToString) -> Option<F>
where
    F: FromStr<Err = E>,
    E: Debug,
{
    let key = key.to_string();

    dotenvy::var(&key).ok().map(|value| {
        value
            .parse()
            .unwrap_or_else(|_| panic!("failed to parse {key}"))
    })
}
