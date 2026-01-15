# log.rs

![Licenses](https://github.com/yonasBSD/log.rs/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/log.rs/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/log.rs/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/log.rs/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/log.rs/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/log.rs/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/log.rs/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_log.rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_log.rs)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_log.rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_log.rs)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_log.rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_log.rs)
<!--[![codecov](https://codecov.io/gh/yonasBSD/log.rs/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/log.rs)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/log.rs/pkgs/container/log.rs)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/log.rs/example.svg)](https://hub.docker.com/r/log.rs/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/log.rs/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/log.rs)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/log.rs/status.svg)](https://deps.rs/repo/github/yonasBSD/log.rs)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/log.rs.svg)](https://github.com/yonasBSD/log.rs/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/log.rs.svg)](https://github.com/yonasBSD/log.rs/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

High performance, general purpose web server.

## Features
- based on high performance Axum web framework
- uses Tower middleware
- Tako web framework optimizations
- built-in OpenAPI support (Swagger UI and Scalar)


## Example

```rust
use anyhow::Result;
use web_server_rs::{*, prelude::*};

async fn hello() -> Response {
    http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::from("Hello World"))
        .unwrap()
}

async fn health() -> impl Responder {
    Json!({ "status": "healthy" })
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: ServerConfig = ServerConfig {
        routes: vec![
            Route {
                method: Method::GET,
                path: "/",
                handler: handler!(hello),
                operation_id: "hello",
                summary: "Hello endpoint",
                description: None,
                tag: "example",
                response_code: 200,
                response_desc: "OK",
            },
            Route {
                method: Method::GET,
                path: "/health",
                handler: handler!(health),
                operation_id: "health",
                summary: "health endpoint",
                description: None,
                tag: "example",
                response_code: 200,
                response_desc: "OK",
            },
        ],
        address: "0.0.0.0",
        port: 3000,
        ..Default::default()
    };

    serve(config).await
}
```
