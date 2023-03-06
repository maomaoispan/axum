# axum

`axum` 是一个专注于人体工学和模块化的 web 框架。

[![Build status](https://github.com/tokio-rs/axum/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/tokio-rs/axum/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/axum)](https://crates.io/crates/axum)
[![Documentation](https://docs.rs/axum/badge.svg)](https://docs.rs/axum)

关于这个库的更多信息可以在[crate 文档][docs]找到.

## 高级特征

- 使用无宏 API 将请求路由到处理程序。
- 使用提取器（extractors）声明式地解析请求。
- 简单并且可以预测的错误处理模型。
- 使用最小模板生成响应。
- 充分利用 `tower` 和 `tower-http` 的中间件生态系统、服务和工具链。

尤其是最后一点是 `axum` 和 `其他框架的区别。axum` 没有自己的中间件系统，而是基于 [`tower::Service`]。这也意味着 `axum` 无偿拥有超时、跟踪、压缩、授权等功能。它还使您能够与使用 `hyper` 或 `tonic` 编写的应用程序共享中间件。

## 使用案例

```rust
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	// 初始化跟踪器
    tracing_subscriber::fmt::init();

    // 基于一个路由构建我们的应用
    let app = Router::new()
        // `GET /` 请求到 `root`服务
        .route("/", get(root))

        // `POST /users` 请求到 `create_user`服务
        .route("/users", post(create_user));

    // 基于 hyper 启动服务
    // `axum::Server` 是基于 `hyper::Server`的二次分装
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// 返回一个静态字符串的最基本处理程序
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // 本参数告诉 axum 把请求体的 JSON 格式解析为 `CreateUser` 类型
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    // 应用的具体业务逻辑
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // 将 user 数据以 `201 Created` 状态码的 JSON 数据格式返回
    (StatusCode::CREATED, Json(user))
}

//  创建用户应用的输入参数 `create_user`
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// 创建用户应用的输出参数 `create_user`
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
```

You can find this [example][readme-example] as well as other example projects in
the [example directory][examples].

See the [crate documentation][docs] for way more examples.

## Performance

`axum` is a relatively thin layer on top of [`hyper`] and adds very little
overhead. So `axum`'s performance is comparable to [`hyper`]. You can find
benchmarks [here](https://github.com/programatik29/rust-web-benchmarks) and
[here](https://web-frameworks-benchmark.netlify.app/result?l=rust).

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Minimum supported Rust version

axum's MSRV is 1.60.

## Examples

The [examples] folder contains various examples of how to use `axum`. The
[docs] also provide lots of code snippets and examples. For full-fledged examples, check out community-maintained [showcases] or [tutorials].

## Getting Help

In the `axum`'s repo we also have a [number of examples][examples] showing how
to put everything together. Community-maintained [showcases] and [tutorials] also demonstrate how to use `axum` for real-world applications. You're also welcome to ask in the [Discord channel][chat] or open a [discussion] with your question.

## Community projects

See [here][ecosystem] for a list of community maintained crates and projects
built with `axum`.

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][contributing] to help you get involved in the
`axum` project.

## License

This project is licensed under the [MIT license][license].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `axum` by you, shall be licensed as MIT, without any
additional terms or conditions.

[readme-example]: https://github.com/tokio-rs/axum/tree/main/examples/readme
[examples]: https://github.com/tokio-rs/axum/tree/main/examples
[docs]: https://docs.rs/axum
[`tower`]: https://crates.io/crates/tower
[`hyper`]: https://crates.io/crates/hyper
[`tower-http`]: https://crates.io/crates/tower-http
[`tonic`]: https://crates.io/crates/tonic
[contributing]: https://github.com/tokio-rs/axum/blob/main/CONTRIBUTING.md
[chat]: https://discord.gg/tokio
[discussion]: https://github.com/tokio-rs/axum/discussions/new?category=q-a
[`tower::service`]: https://docs.rs/tower/latest/tower/trait.Service.html
[ecosystem]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md
[showcases]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#project-showcase
[tutorials]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials
[license]: https://github.com/tokio-rs/axum/blob/main/axum/LICENSE
