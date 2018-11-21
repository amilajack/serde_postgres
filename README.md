# Serde Postgres
[![Build status](https://img.shields.io/travis/1aim/serde_postgres.svg?branch=master)](https://travis-ci.org/1aim/serde_postgres)
[![Crate](https://img.shields.io/crates/d/serde_postgres.svg)](https://crates.io/crates/serde_postgres)
[![Lines Of Code](https://tokei.rs/b1/github/1aim/serde_postgres?category=code)](https://github.com/Aaronepower/tokei)
[![Documentation](https://docs.rs/serde_postgres/badge.svg)](https://docs.rs/serde_postgres/)

Easily deserialize rows from [`postgres`](//docs.rs/postgres) into
arbitrary structs. (Only deserialization is supported).

```rust
extern crate serde;
extern crate serde_derive;
extern crate serde_postgres;
extern crate postgres;

use std::error::Error;

use serde_derive::Deserialize;
use postgres::{Connection, TlsMode};

#[derive(Clone, Debug, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn main() -> Result<(), Box<Error>> {
    let connection = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None)?;

    connection.execute("CREATE TABLE IF NOT EXISTS Person (
        name VARCHAR NOT NULL,
        age INT NOT NULL
    )", &[])?;

    connection.execute("INSERT INTO Person (name, age) VALUES ($1, $2)",
    &[&"Jane", &23])?;

    connection.execute("INSERT INTO Person (name, age) VALUES ($1, $2)",
    &[&"Alice", &32])?;
    
    let rows = connection.query("SELECT name, age FROM Person", &[])?;

    let people: Vec<Person> = serde_postgres::from_rows(&rows)?;

    for person in people {
        println!("{:?}", person);
    }

    Ok(())
}
```
