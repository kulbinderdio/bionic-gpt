use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use http::header;

// Use the i18n crate directly
use i18n;

pub async fn i18n_middleware(request: Request, next: Next) -> Response {
    // Extract Accept-Language header
    if let Some(accept_language) = request.headers().get(header::ACCEPT_LANGUAGE) {
        if let Ok(accept_language) = accept_language.to_str() {
            // Ignore errors from language detection - we'll fall back to default
            let _ = i18n::set_language_from_accept_language(accept_language);
        }
    }

    // Continue with the request
    next.run(request).await
}
