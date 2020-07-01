# lovecraft-api

[![LICENSE](https://img.shields.io/badge/License-MIT_or_Apache_2.0-green.svg)](https://github.com/GiorgiBeriashvili/lovecraft-api#License
"Project's LICENSE section")

## Description

An API for H.P. Lovecraft's Works' [Electronic Texts](https://www.hplovecraft.com/writings/texts/).

---

## Usage

In order for the server to function properly, you need initialize the database and set some environment variables.

Database (RDBMS being SQLite in this case) can have a table defined in the following way:

```sql
CREATE TABLE IF NOT EXISTS manuscript (
    id          INTEGER PRIMARY KEY NOT NULL,
    category    TEXT NOT NULL,
    title       TEXT NOT NULL UNIQUE,
    content     TEXT NOT NULL,
    description TEXT NOT NULL
);
```

You can then insert any data that you like (preferably public domain H.P. Lovecraft's works), like so:

```sql
INSERT INTO manuscript (category, title, content, description)
VALUES ('Category', 'The Title', 'The content of the manuscript.', 'The description of the manuscript.');
```

As for the environment variables, here is an example (`.env` file):

```sh
HOST=127.0.0.1
PORT=3000
DATABASE_URL="sqlite://database/manuscript.db"
RUST_LOG=lovecraft_api=info,actix=info,actix_web=info
CERTIFICATE="cert.pem"
CERTIFICATE_KEY="key.pem"
```

Regarding routes, please visit the `/api/v1/manuscripts` endpoint for the main content!

Available routes:
- GET `/manuscripts` -> array of entry objects consisting of: id, category, title, description
- GET `/manuscripts/{id}` -> manuscript object consisting of: id, category, title, content

For the complete API design and documentation (does not fully match the application's functionality in terms of error results), you can take a look at the [OpenAPI Specification based API documentation](https://github.com/GiorgiBeriashvili/lovecraft-api/blob/master/api.yaml "API documentation").

Finally, `mkcert` is used to issue local certificates to enable `https` (by first running `mkcert -install` and then `mkcert -key-file key.pem -cert-file cert.pem localhost 127.0.0.1 ::1`). You can disable the certificate functionality altogether by putting the following code instead of the existing one in `main.rs` and then removing the unnecessary leftover code:

```rust
server.bind(format!("{}:{}", host, port))?
```

Here are the sample responses:

- `/manuscripts`:

```json
[
    {
        "id": 1,
        "category": "Category",
        "title": "The Title",
        "description": "The description of the manuscript."
    },
    {
        "id": 2,
        "category": "Category",
        "title": "The Title",
        "description": "The description of the manuscript."
    }
]
```

- `/manuscripts/1`:

```json
{
    "id": 1,
    "category": "Category",
    "title": "The Title",
    "content": "The content of the manuscript."
}
```

---

## Building

In order to build lovecraft-api, you need to have [Rust](https://www.rust-lang.org
"Rust programming language's official website") programming language installed
on your system. To install Rust (alongside
[Cargo](https://doc.rust-lang.org/stable/cargo "The Cargo Book"), which comes bundled with
Rust), it's best to follow the [official installation
steps](https://www.rust-lang.org/tools/install "Official guide to install
Rust").

It is preferred to build the project with Rust version 1.44.1 (c7087fe00 2020-06-17).

```sh
# Clone the repository
git clone https://github.com/GiorgiBeriashvili/lovecraft-api
cd lovecraft-api

# Build the debug version
cargo build
```

---

## License

lovecraft-api is licensed under either of the following, at your option:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/GiorgiBeriashvili/lovecraft-api/blob/master/LICENSE-APACHE "Copy of the Apache license (version 2.0)"))
- MIT License ([LICENSE-MIT](https://github.com/GiorgiBeriashvili/lovecraft-api/blob/master/LICENSE-MIT "Copy of the MIT license"))
