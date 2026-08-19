use poem::{Route, Endpoint, EndpointExt};

/// Упрощённое приложение для сборки маршрутов.
pub struct App {
    route: Route,
}

impl App {
    /// Создаёт новый экземпляр `App`.
    pub fn new() -> Self {
        Self { route: Route::new() }
    }

    // ---- Все методы маршрутизации ----

    /// Добавляет GET-маршрут.
    pub fn get<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::get(endpoint));
        self
    }

    /// Добавляет POST-маршрут.
    pub fn post<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::post(endpoint));
        self
    }

    /// Добавляет PUT-маршрут.
    pub fn put<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::put(endpoint));
        self
    }

    /// Добавляет DELETE-маршрут.
    pub fn delete<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::delete(endpoint));
        self
    }

    /// Добавляет PATCH-маршрут.
    pub fn patch<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::patch(endpoint));
        self
    }

    /// Добавляет OPTIONS-маршрут.
    pub fn options<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::options(endpoint));
        self
    }

    /// Добавляет HEAD-маршрут.
    pub fn head<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::head(endpoint));
        self
    }

    /// Добавляет TRACE-маршрут.
    pub fn trace<E: Endpoint + 'static>(mut self, path: &str, endpoint: E) -> Self {
        self.route = self.route.at(path, poem::trace(endpoint));
        self
    }

    /// Применяет middleware ко всему приложению.
    ///
    /// # Пример
    /// ```rust
    /// use jetapi::prelude::*;
    /// let app = App::new()
    ///     .get("/", handler)
    ///     .with(Cors::new())
    ///     .into_route();
    /// ```
    pub fn with<M>(mut self, middleware: M) -> Self
    where
        M: poem::middleware::Middleware<Route>,
        M::Output: Into<Route>,
    {
        self.route = self.route.with(middleware).into();
        self
    }

    /// Завершает сборку и возвращает готовый `Route`.
    ///
    /// Этот метод вызывается после добавления всех маршрутов и middleware.
    pub fn into_route(self) -> Route {
        self.route
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}