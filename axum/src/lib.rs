#![cfg_attr(nightly_error_messages, feature(rustc_attrs))]
//! axum 是一个专注于人体工学和模块化的 web 框架。
//! 
//! *axum is a web application framework that focuses on ergonomics and modularity.*
//! 
//!
//! # 内容清单 *Table of contents* 
//! 
//!
//! - [高级特征 *High-level  features*](#high-level-features)
//! - [兼容性 *Compatibility*](#compatibility) 
//! - [用例 *Example*](#example) 
//! - [路由 *Routing*](#routing) 
//! - [Handlers](#handlers) 
//! - [Extractors](#extractors) 
//! - [Responses](#responses) 
//! - [错误处理 *Error handling*](#error-handling) 
//! - [中间件 *Middleware*](#middleware) 
//! - [Handlers 之间的数据共享 *Sharing state with handlers*](#sharing-state-with-handlers) 
//! - [axum 构建集成 *Building integrations for axum*](#building-integrations-for-axum) 
//! - [必要的依赖 *Required dependencies*](#required-dependencies) 
//! - [用例集合 *Examples*](#examples) 
//! - [Feature flags](#feature-flags) 
//!
//! # 高级特征 *High-level features*
//!
//! - 使用无宏 API 将请求路由到 handlers。 *Route requests to handlers with a macro-free API.* 
//! - 使用 extractors 声明式地解析请求。 *Declaratively parse requests using extractors.*
//! - 简单并且可以预测的错误处理模型。 *Simple and predictable error handling model.*
//! - 使用最小模板生成响应。*Generate responses with minimal boilerplate.*
//! - 充分利用 [`tower`] 和 [`tower-http`] 的中间件生态系统、服务和工具链。
//! *Take full advantage of the [`tower`] and [`tower-http`] ecosystem of
//!   middleware, services, and utilities.* 
//!
//! 特别是最后一点是 `axum` 和 `其他框架的区别。axum` 没有自己的中间件系统，
//! 而是基于 [`tower::Service`]。这也意味着 `axum` 轻松拥有超时、跟踪、压缩、授权等中间件功能。
//! 它还能让您同基于 `hyper` 或 `tonic` 编写的应用共享中间件。
//! 
//! *In particular, the last point is what sets `axum` apart from other frameworks.
//! `axum` doesn't have its own middleware system but instead uses
//! [`tower::Service`]. This means `axum` gets timeouts, tracing, compression,
//! authorization, and more, for free. It also enables you to share middleware with
//! applications written using [`hyper`] or [`tonic`].*
//!
//! # 兼容性 *Compatibility*
//!
//! axum 是基于 [tokio] 和 [hyper] 设计而运行的。至少当前运行时和传输层是相互独立的，但这并不是主要目标。
//! 
//! *axum is designed to work with [tokio] and [hyper]. Runtime and
//! transport layer independence is not a goal, at least for the time being.*
//!
//! # 简单用例 *Example*
//!
//! The "Hello, World!" of axum is:
//!
//! ```rust,no_run
//! use axum::{
//!     routing::get,
//!     Router,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // 基于单个路由构建我们的程序 
//!     // build our application with a single route
//!     let app = Router::new().route("/", get(|| async { "Hello, World!" }));
//!
//!     // run it with hyper on localhost:3000
//!     // 基于 hyper 将应用运行在 localhost:3000 端口上
//!     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! ```
//! 
//! 注意：使用 `#[tokio::main]` 宏，需要你开启 tokio 的 `macros` 和 `rt-multi-thread` 这两个 Feature，
//! 或者只使用 `full` 开启全部 Feature (`cargo add tokio --features macros,rt-multi-thread`)。
//!
//! *Note using `#[tokio::main]` requires you enable tokio's `macros` and `rt-multi-thread` features
//! or just `full` to enable all features (`cargo add tokio --features macros,rt-multi-thread`).*
//!
//! # 路由 *Routing*
//!
//! [`Router`] 路由用于配置哪些路径指向哪些服务：
//! 
//! *[`Router`] is used to setup which paths goes to which services:*
//!
//! ```rust
//! use axum::{Router, routing::get};
//!
//! // our router
//! // 我们的路由
//! let app = Router::new()
//!     .route("/", get(root))
//!     .route("/foo", get(get_foo).post(post_foo))
//!     .route("/foo/bar", get(foo_bar));
//!
//! // which calls one of these handlers
//! // 调用这些处理器的其中一个
//! async fn root() {}
//! async fn get_foo() {}
//! async fn post_foo() {}
//! async fn foo_bar() {}
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//!
//! 关于路由的更多细节请查看 [`Router`]章节。
//! 
//! *See [`Router`] for more details on routing.*
//!
//! # Handlers
//!
#![doc = include_str!("docs/handlers_intro.md")]
//!
//! 关于 handlers 的更多细节请查看 [`handler`](crate::handler) 章节。
//! 
//! *See [`handler`](crate::handler) for more details on handlers.*
//!
//! # Extractors
//! 
//! 一个 extractor 是一个实现了 [`FromRequest`] or [`FromRequestParts`] 的类型。
//! Extractors 是你如何分离输入请求以获取处理程序所需部分的方式。
//!
//! *An extractor is a type that implements [`FromRequest`] or [`FromRequestParts`]. Extractors are
//! how you pick apart the incoming request to get the parts your handler needs.*
//!
//! ```rust
//! use axum::extract::{Path, Query, Json};
//! use std::collections::HashMap;
//!
//! // `Path` gives you the path parameters and deserializes them.
//! // `Paht` 给您提供路径参数，并且反序列化它们。
//! async fn path(Path(user_id): Path<u32>) {}
//!
//! // `Query` gives you the query parameters and deserializes them.
//! // `Query` 给您提供查询参数，并且反序列化它们。
//! async fn query(Query(params): Query<HashMap<String, String>>) {}
//!
//! // Buffer the request body and deserialize it as JSON into a `serde_json::Value`. 
//! // 缓冲请求主体并将其作为 JSON 反序列化为 `serde_json::Value`。
//! // 
//! // `Json` supports any type that implements `serde::Deserialize`.
//! // `Json` 支持任何实现了 `serde::Deserialize` 的类型。
//! async fn json(Json(payload): Json<serde_json::Value>) {}
//! ```
//!
//! 关于 extractors 的更多详情请查看 [`extract`](crate::extract) 章节。
//! *See [`extract`](crate::extract) for more details on extractors.*
//!
//! # Responses
//! 任何实现了 [`IntoResponse`] 特征的对象均能从 handlers 中返回。
//! *Anything that implements [`IntoResponse`] can be returned from handlers.*
//!
//! ```rust,no_run
//! use axum::{
//!     body::Body,
//!     routing::get,
//!     response::Json,
//!     Router,
//! };
//! use serde_json::{Value, json};
//! 
//! // `&'static str` 返回一个 `content-type: text/plain; charset=utf-8` 的 `200 OK` 响应体。
//! // `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
//! async fn plain_text() -> &'static str {
//!     "foo"
//! }
//!
//! // `Json`可以从任何实现了 `serde::Serialize` 特征的对象得到一个 content-type 为 `application/json` 的响应体
//! // `Json` gives a content-type of `application/json` and works with any type
//! // that implements `serde::Serialize`
//! async fn json() -> Json<Value> {
//!     Json(json!({ "data": 42 }))
//! }
//!
//! let app = Router::new()
//!     .route("/plain_text", get(plain_text))
//!     .route("/json", get(json));
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//! 关于构建响应体的更多细节请查看 [`response`](crate::response) 章节。
//! 
//! *See [`response`](crate::response) for more details on building responses.*
//!
//! # 错误处理 Error handling
//!
//! axum 的目标是实现一套简单的、可预测的错误处理模型。
//! 这意味着将错误转换为响应很简单，并且可以保证所有错误都得到处理
//! 
//! *axum aims to have a simple and predictable error handling model. That means
//! it is simple to convert errors into responses and you are guaranteed that
//! all errors are handled.*
//!
//! 更多关于 axum's 的错误处理模型和怎么优雅地处理错误，
//! 请查看 [`error_handling`](crate::error_handling) 章节。
//! 
//! *See [`error_handling`](crate::error_handling) for more details on axum's
//! error handling model and how to handle errors gracefully.*
//!
//! # 中间件 *Middleware*
//!
//! 编写 axum 的中间件有许多种方式。详细信息请查看 [`middleware`](crate::middleware) 章节。
//! 
//! *There are several different ways to write middleware for axum. 
//! See [`middleware`](crate::middleware) for more details.*
//!
//! # handlers 状态共享 *Sharing state with handlers*
//!
//! 在处理程序之间共享一些状态是很常见的。
//! 例如：数据库连接池或者连接到其他服务的客户端均需要共享。
//! 
//! *It is common to share some state between handlers. For example, a
//! pool of database connections or clients to other services may need to
//! be shared.*
//!
//! 常规的做法有以下几种：
//! 
//! *The three most common ways of doing that are:*
//! - 使用 [`State`] extractor *Using the [`State`] extractor*
//! - 使用请求扩展 *Using request extensions*
//! - 使用闭包捕获 *Using closure captures*
//!
//! ## 使用 [`State`] extractor *Using the [`State`] extractor*
//!
//! ```rust,no_run
//! use axum::{
//!     extract::State,
//!     routing::get,
//!     Router,
//! };
//! use std::sync::Arc;
//!
//! struct AppState {
//!     // ...
//! }
//!
//! let shared_state = Arc::new(AppState { /* ... */ });
//!
//! let app = Router::new()
//!     .route("/", get(handler))
//!     .with_state(shared_state);
//!
//! async fn handler(
//!     State(state): State<Arc<AppState>>,
//! ) {
//!     // ...
//! }
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//!
//! 如果可能，您应该更倾向使用 [`State`]，因为它更安全。缺点是它不如请求扩展灵活。
//! 
//! *You should prefer using [`State`] if possible since it's more type safe. The downside is that
//! it's less dynamic than request extensions.*
//!
//! 关于访问状态的更多详情，请查看 [`State`] 章节。
//! 
//! *See [`State`] for more details about accessing state.*
//!
//! ## 使用请求扩展 *Using request extensions*
//!
//! 另外一种在 handlers 中提取状态的方法是把 [`Extension`](crate::extract::Extension) 
//! 作为 layer 和 extractor：
//! 
//! *Another way to extract state in handlers is using [`Extension`](crate::extract::Extension) as
//! layer and extractor:*
//!
//! ```rust,no_run
//! use axum::{
//!     extract::Extension,
//!     routing::get,
//!     Router,
//! };
//! use std::sync::Arc;
//!
//! struct AppState {
//!     // ...
//! }
//!
//! let shared_state = Arc::new(AppState { /* ... */ });
//!
//! let app = Router::new()
//!     .route("/", get(handler))
//!     .layer(Extension(shared_state));
//!
//! async fn handler(
//!     Extension(state): Extension<Arc<AppState>>,
//! ) {
//!     // ...
//! }
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//! 
//! 这种方式的缺点是，当你尝试提取一个不存在的 extension 时候，会得到一个运行时错误（特别是 `500 Internal Server Error`响应），
//! 原因也许是你忘记添加中间件，或者你正在提取一个错误的类型。
//!
//! *The downside to this approach is that you'll get runtime errors
//! (specifically a `500 Internal Server Error` response) if you try and extract
//! an extension that doesn't exist, perhaps because you forgot to add the
//! middleware or because you're extracting the wrong type.*
//!
//! ## 使用闭包捕获 *Using closure captures*
//!
//! 状态也能直接通过闭包捕获的方式传递到 handlers：
//! 
//! *State can also be passed directly to handlers using closure captures:*
//!
//! ```rust,no_run
//! use axum::{
//!     Json,
//!     extract::{Extension, Path},
//!     routing::{get, post},
//!     Router,
//! };
//! use std::sync::Arc;
//! use serde::Deserialize;
//!
//! struct AppState {
//!     // ...
//! }
//!
//! let shared_state = Arc::new(AppState { /* ... */ });
//!
//! let app = Router::new()
//!     .route(
//!         "/users",
//!         post({
//!             let shared_state = Arc::clone(&shared_state);
//!             move |body| create_user(body, shared_state)
//!         }),
//!     )
//!     .route(
//!         "/users/:id",
//!         get({
//!             let shared_state = Arc::clone(&shared_state);
//!             move |path| get_user(path, shared_state)
//!         }),
//!     );
//!
//! async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) {
//!     // ...
//! }
//!
//! async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>) {
//!     // ...
//! }
//!
//! #[derive(Deserialize)]
//! struct CreateUserPayload {
//!     // ...
//! }
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//!
//! 这种方法的缺点是它比使用 State 或扩展更冗长。
//! 
//! *The downside to this approach is that it's a little more verbose than using
//! [`State`] or extensions.*
//!
//! # axum 的构建集成 *Building integrations for axum*
//!
//! 想要提供 FromRequest、 FromRequestPart 或者 IntoResponse 实现的库作者尽量基于 [`axum-core`]，而不是 `axum`。
//! [`axum-core`] 包含核心类型和特征，不太可能收到破坏性变动。
//! 
//! *Libraries authors that want to provide [`FromRequest`], [`FromRequestParts`], or
//! [`IntoResponse`] implementations should depend on the [`axum-core`] crate, instead of `axum` if
//! possible. [`axum-core`] contains core types and traits and is less likely to receive breaking
//! changes.*
//!
//! # 必要依赖库 *Required dependencies*
//!
//! 要使用 axum，您还必须引入一些依赖库：
//! 
//! *To use axum there are a few dependencies you have to pull in as well:*
//!
//! ```toml
//! [dependencies]
//! axum = "<latest-version>"
//! hyper = { version = "<latest-version>", features = ["full"] }
//! tokio = { version = "<latest-version>", features = ["full"] }
//! tower = "<latest-version>"
//! ```
//!
//! hyper 和 tokio 的“full” feature 并非绝对必要，但这是最简单的入门方式。
//! 
//! *The `"full"` feature for hyper and tokio isn't strictly necessary but it's
//! the easiest way to get started.*
//!
//! 注意 [`hyper::Server`] 被 axum 重新导出，所以非必要你不需要显式的引入 hyper。
//! 
//! *Note that [`hyper::Server`] is re-exported by axum so if that's all you need
//! then you don't have to explicitly depend on hyper.*
//!
//! 
//! Tower 也不是绝对必要的，但有对测试有用。
//! 在代码仓库查看测试案例，学习更多关于 axum 应用的测试。
//! 
//! *Tower isn't strictly necessary either but helpful for testing. See the
//! testing example in the repo to learn more about testing axum apps.*
//!
//! # 用例集合 *Examples*
//!
//! axum 官方仓库包括了[大量的用例][examples]，展示了如何将所有的程序碎片进行集成。
//! 
//! *The axum repo contains [a number of examples][examples] that show how to put all the
//! pieces together.*
//!
//! # Feature flags
//!
//! axum 使用一组[feature flags]来减少已编译和可选依赖项的数量。
//! 
//! *axum uses a set of [feature flags] to reduce the amount of compiled and
//! optional dependencies.*
//!
//! 以下可选 features 均是可用的：
//! 
//! *The following optional features are available:*
//!
//! 名称 *Name* | 描述 *Description* | 默认？ *Default?*
//! ---|---|---
//! `headers`           | 启用通过 [`TypedHeader`] 提取类型化的标头                                   <br />*Enables extracting typed headers via [`TypedHeader`]*                                              | No
//! `http1`             | 启用 hyper's `http1` feature                                             <br />*Enables hyper's `http1` feature*                                                                  | Yes
//! `http2`             | 启用 hyper's `http2` feature                                             <br />*Enables hyper's `http2` feature*                                                                  | No
//! `json`              | 启用 [`Json`] 类型和一些类似的便利功能                                       <br />*Enables the [`Json`] type and some similar convenience functionality*                             | Yes
//! `macros`            | 启用宏工具                                                                <br />*Enables optional utility macros*                                                                   | No
//! `matched-path`      | 启用捕获每个请求的路由器路径和 [`MatchedPath`] 提取器                          <br />*Enables capturing of every request's router path and the [`MatchedPath`] extractor*                | Yes
//! `multipart`         | 启用通过 [`Multipart`] 解析 `multipart/form-data` 请求                     <br />*Enables parsing `multipart/form-data` requests with [`Multipart`]*                                 | No
//! `original-uri`      | 启用通过 [`OriginalUri`] extractor 捕获每个请求的 original URI              <br />*Enables capturing of every request's original URI and the [`OriginalUri`] extractor*               | Yes
//! `tokio`             | 启用 tokio 作为依赖项和 axum::Server、SSE 和 extract::connect_info 类型      <br />*Enables `tokio` as a dependency and `axum::Server`, `SSE` and `extract::connect_info` types.*      | Yes
//! `tower-log`         | 启用 `tower` 的 `log` feature                                             <br />*Enables `tower`'s `log` feature*                                                                   | Yes
//! `ws`                | 通过[`extract::ws`] 启用 WebSockets                                        <br />*Enables WebSockets support via [`extract::ws`]*                                                   | No
//! `form`              | 启用 `Form` extractor                                                     <br />*Enables the `Form` extractor*                                                                     | Yes
//! `query`             | 启用`Query` extractor                                                     <br />*Enables the `Query` extractor                                                                     | Yes
//!
//! [`TypedHeader`]: crate::extract::TypedHeader
//! [`MatchedPath`]: crate::extract::MatchedPath
//! [`Multipart`]: crate::extract::Multipart
//! [`OriginalUri`]: crate::extract::OriginalUri
//! [`tower`]: https://crates.io/crates/tower
//! [`tower-http`]: https://crates.io/crates/tower-http
//! [`tokio`]: http://crates.io/crates/tokio
//! [`hyper`]: http://crates.io/crates/hyper
//! [`tonic`]: http://crates.io/crates/tonic
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section
//! [`IntoResponse`]: crate::response::IntoResponse
//! [`Timeout`]: tower::timeout::Timeout
//! [examples]: https://github.com/tokio-rs/axum/tree/main/examples
//! [`Router::merge`]: crate::routing::Router::merge
//! [`axum::Server`]: hyper::server::Server
//! [`Service`]: tower::Service
//! [`Service::poll_ready`]: tower::Service::poll_ready
//! [`Service`'s]: tower::Service
//! [`tower::Service`]: tower::Service
//! [tower-guides]: https://github.com/tower-rs/tower/tree/master/guides
//! [`Uuid`]: https://docs.rs/uuid/latest/uuid/
//! [`FromRequest`]: crate::extract::FromRequest
//! [`FromRequestParts`]: crate::extract::FromRequestParts
//! [`HeaderMap`]: http::header::HeaderMap
//! [`Request`]: http::Request
//! [customize-extractor-error]: https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/main.rs
//! [axum-macros]: https://docs.rs/axum-macros
//! [`debug_handler`]: https://docs.rs/axum-macros/latest/axum_macros/attr.debug_handler.html
//! [`Handler`]: crate::handler::Handler
//! [`Infallible`]: std::convert::Infallible
//! [load shed]: tower::load_shed
//! [`axum-core`]: http://crates.io/crates/axum-core
//! [`State`]: crate::extract::State

#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

#[macro_use]
pub(crate) mod macros;

mod boxed;
mod extension;
#[cfg(feature = "form")]
mod form;
#[cfg(feature = "json")]
mod json;
mod service_ext;
#[cfg(feature = "headers")]
mod typed_header;
mod util;

pub mod body;
pub mod error_handling;
pub mod extract;
pub mod handler;
pub mod middleware;
pub mod response;
pub mod routing;

#[cfg(test)]
mod test_helpers;

#[doc(no_inline)]
pub use async_trait::async_trait;
#[cfg(feature = "headers")]
#[doc(no_inline)]
pub use headers;
#[doc(no_inline)]
pub use http;
#[cfg(feature = "tokio")]
#[doc(no_inline)]
pub use hyper::Server;

#[doc(inline)]
pub use self::extension::Extension;
#[doc(inline)]
#[cfg(feature = "json")]
pub use self::json::Json;
#[doc(inline)]
pub use self::routing::Router;

#[doc(inline)]
#[cfg(feature = "headers")]
pub use self::typed_header::TypedHeader;

#[doc(inline)]
#[cfg(feature = "form")]
pub use self::form::Form;

#[doc(inline)]
pub use axum_core::{BoxError, Error, RequestExt, RequestPartsExt};

#[cfg(feature = "macros")]
pub use axum_macros::debug_handler;

pub use self::service_ext::ServiceExt;

#[cfg(test)]
use axum_macros::__private_axum_test as test;
