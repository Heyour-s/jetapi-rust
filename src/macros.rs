//! Декларативные макросы JetAPI

/// Собирает все маршруты из обработчиков, помеченных макросами `#[get]`, `#[post]` и т.д.
///
/// Каждый обработчик должен иметь публичную функцию `__register_route`, которая принимает
/// `axum::Router` и возвращает новый `Router` с добавленным маршрутом.
///
/// # Пример
///
/// ```rust
/// use jetapi::{routes, get};
///
/// #[get("/")]
/// async fn home() -> &'static str {
///     "Hello, world!"
/// }
///
/// #[get("/about")]
/// async fn about() -> &'static str {
///     "About us"
/// }
///
/// let router = routes!(home, about);
/// ```
#[macro_export]
macro_rules! routes {
    ($($handler:ident),* $(,)?) => {{
        let mut router = ::axum::Router::new();
        $(
            router = $handler::__register_route(router);
        )*
        router
    }};
}