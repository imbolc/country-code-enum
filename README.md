# country-code-enum

[![License](https://img.shields.io/crates/l/country-code-enum.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/country-code-enum.svg)](https://crates.io/crates/country-code-enum)
[![Docs.rs](https://docs.rs/country-code-enum/badge.svg)](https://docs.rs/country-code-enum)

Copyable Serde and Sqlx compatible country codes

```rust
use country_code_enum::CountryCode;

let argentina: CountryCode = "AR".parse().unwrap();
assert_eq!(argentina, CountryCode::AR);
assert_eq!(argentina.as_ref(), "AR");
assert_eq!(argentina.name(), "Argentina");
```

## Serde

```rust
use country_code_enum::CountryCode;

#[cfg(feature = "serde")]
{
    let argentina: CountryCode = serde_json::from_str("\"AR\"").unwrap();
    assert_eq!(argentina, CountryCode::AR);
    assert_eq!(serde_json::to_string(&argentina).unwrap(), "\"AR\"");
}
```

## Sqlx

```rust,ignore
use country_code_enum::CountryCode;

async fn sqlx_example(pool: sqlx::PgPool) {
    let argentina = sqlx::query_scalar!(r#"SELECT 'AR'::varchar as "val: CountryCode""#)
        .fetch_one(&pool)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(argentina, CountryCode::AR);

    let s = sqlx::query_scalar!("SELECT $1::varchar", argentina as _)
        .fetch_one(&pool)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(s, "AR");
}
```


## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything


## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]: https://github.com/imbolc/country-code-enum/blob/main/pre-commit.sh
[license]: https://github.com/imbolc/country-code-enum/blob/main/LICENSE
