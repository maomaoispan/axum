在 axum 中，一个 handler 是一个异步函数，该函数接受一个或多个["extractors"](crate::extract)作为参数，返回值是可以转换为 [response](crate::response) 的任何对象。

*In axum a "handler" is an async function that accepts zero or more
["extractors"](crate::extract) as arguments and returns something that
can be converted [into a response](crate::response).*


Handlers 是您的应用主要逻辑所在的地方，而 axum 应用是通过 handlers 和路由构建的。

*Handlers is where your application logic lives and axum applications are built
by routing between handlers.*



[`debug_handler`]: https://docs.rs/axum-macros/latest/axum_macros/attr.debug_handler.html
