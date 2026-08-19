use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;

/// Добавляет Swagger UI к роутеру по пути /docs.
pub fn with_openapi(router: Router, api: OpenApi) -> Router {
    router.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api))
}

/// Пустая схема для примера.
#[derive(OpenApi)]
#[openapi(paths(), components(schemas()))]
pub struct ApiDoc;