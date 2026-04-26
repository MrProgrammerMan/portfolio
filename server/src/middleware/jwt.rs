use axum::{extract::Request, middleware::Next, response::IntoResponse};

pub async fn jwt_validation(req: Request, next: Next) -> impl IntoResponse {
    next.run(req).await // TODO: Not yet implemented
}
