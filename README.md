<h1 align="center">Actix Json Response</h1>
<div align="center">
 <strong>
   A library that exposes a helper type for Json responses in Actix-Web.
 </strong>
</div>

<br />

[![Continuous Integration](https://github.com/moy2010/actix-json-response/actions/workflows/ci.yml/badge.svg)](https://github.com/moy2010/actix-json-response/actions/workflows/ci.yml) [![license-badge][]][license] [![rust-version-badge][]][rust-version]

# Getting started

## How to install

Add `actix-json-response` to your dependencies:

```toml
[dependencies]
# ...
actix-web = "4"
actix-json-response = "0.1.3"
```

## Quickstart

`actix-json-response` exposes the `JsonResponse` type which implements Actix's `Responder` trait. It is generic and receives a type parameter that must implement Serde's `Serialize` trait:

```rust,compile_fail
use actix_web::{get, web, Result};
use actix_json_response::JsonResponse;
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<JsonResponse<MyObj>> {
    let my_obj = MyObj {
        name: name.to_string(),
    };
    Ok(my_obj.into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

By default, the response will have status code `200`. If you need the response to have a different status code, you can use the `with_status_code` method that receives an Actix's `StatusCode`:

```rust
use actix_web::http::StatusCode;

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<JsonResponse<MyObj>> {
    let my_obj = MyObj {
        name: name.to_string(),
    };
    Ok(JsonResponse::from(my_obj).with_status_code(StatusCode::CREATED)) // The response will have status code 201 in this case
}
```

# License

Distributed under the terms of [MIT license](./LICENSE-MIT) and [Apache license](./LICENSE-APACHE).


[cargo-badge]: https://img.shields.io/crates/v/actix-json-response.svg?style=flat-square
[cargo]: https://crates.io/crates/actix-json-response
[license-badge]: https://img.shields.io/badge/license-MIT/Apache--2.0-lightgray.svg?style=flat-square
[license]: #license
[rust-version-badge]: https://img.shields.io/badge/rust-1.15+-blue.svg?style=flat-square
[rust-version]: .travis.yml#L5