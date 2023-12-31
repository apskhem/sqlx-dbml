# SQLx DBML

#### Database Markup Language (DBML) Code Generation for SQLx Structs.

[![crate](https://img.shields.io/crates/v/sqlx-dbml.svg)](https://crates.io/crates/sqlx-dbml)
![MSRV](https://img.shields.io/badge/rustc-1.59+-ab6000.svg)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/sqlx-dbml.svg)
![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)

## Why DBML?

DBML (Database Markup Language) is an open-source DSL language designed to define and document database schemas and structures. It is designed to be simple, consistent and highly-readable.

Read more: [Official docs](https://www.dbml.org/home/)

This project aims to make use of DBML as a language for generating SQLx structs.

## Output

Below is the example of generating DBML into SQLx structs.

```dbml
Table user {
  id integer [pk]
  username varchar
  role varchar
}
```

```rust
//! Generated by sqlx-dbml 0.1.0

pub mod user {
  use sqlx::*;

  #[derive(FromRow)]
  pub struct Model {
    pub id: i32,
    pub username: String,
    pub role: String,
  }
}

```

## How to use it?

```rust
use std::{ffi::OsString, error::Error};

use sqlx_dbml::{compiler::config::Config, *};

fn main() -> Result<(), Box<dyn Error>> {
  compile(Config {
    in_path: OsString::from("path/to/file.dbml"),
    out_path: OsString::from("path/to/out/mod.rs"),
    target: compiler::config::Target::Postgres,
    ..Default::default()
  })
}

```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Always welcome you to participate, contribute and together.
