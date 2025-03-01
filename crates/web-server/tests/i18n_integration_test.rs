use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;
use std::sync::Arc;

// Import the i18n crate directly
use i18n::{get_message, get_message_with_args, set_language_from_accept_language};

// Test handler that returns translated content
async fn test_handler() -> Response<Body> {
    let welcome = get_message("common-welcome");
    let dashboard = get_message("dashboard-title");
    let content = format!("<html><body><h1>{}</h1><h2>{}</h2></body></html>", welcome, dashboard);
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(content))
        .unwrap()
}

#[tokio::test]
async fn test_language_detection_and_rendering() {
    // Create test app
    let app = Router::new().route("/test", get(test_handler));

    // Test with English
    let request = Request::builder()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "en")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("en");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with Spanish
    let request = Request::builder()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "es")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("es");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Bienvenido a BionicGPT"));
    assert!(body_str.contains("Panel de control"));

    // Test with French
    let request = Request::builder()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "fr")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("fr");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Bienvenue à BionicGPT"));
    assert!(body_str.contains("Tableau de bord"));
}

// Test handler that returns translated content with arguments
async fn test_handler_with_args() -> Response<Body> {
    let username = "TestUser";
    let count = "5";

    let welcome = get_message_with_args("dashboard-welcome", username, count);
    let stats = get_message_with_args("dashboard-stats", username, count);

    let content = format!("<html><body><p>{}</p><p>{}</p></body></html>", welcome, stats);
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(content))
        .unwrap()
}

#[tokio::test]
async fn test_translation_with_arguments() {
    // Create test app
    let app = Router::new().route("/test-args", get(test_handler_with_args));

    // Test with English
    let request = Request::builder()
        .uri("/test-args")
        .header(header::ACCEPT_LANGUAGE, "en")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("en");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT, TestUser!"));
    assert!(body_str.contains("You have 5 active projects"));

    // Test with Spanish
    let request = Request::builder()
        .uri("/test-args")
        .header(header::ACCEPT_LANGUAGE, "es")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("es");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Bienvenido a BionicGPT, TestUser!"));
    assert!(body_str.contains("Tienes 5 proyectos activos"));
}

#[tokio::test]
async fn test_edge_cases() {
    // Create test app
    let app = Router::new().route("/test", get(test_handler));

    // Test with unsupported language (should fall back to English)
    let request = Request::builder()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "ja")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("ja");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with invalid language tag (should fall back to English)
    let request = Request::builder()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "invalid-language-tag")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("invalid-language-tag");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with empty Accept-Language header (should use default language)
    let request = Request::builder()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "")
        .body(Body::empty())
        .unwrap();

    // Set the language based on the Accept-Language header
    let _ = set_language_from_accept_language("");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with no Accept-Language header (should use default language)
    let request = Request::builder()
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    // Reset to English
    let _ = set_language_from_accept_language("en");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with missing translation key (should return the key itself)
    // Create a handler that uses a non-existent key
    async fn missing_key_handler() -> Response<Body> {
        let missing = get_message("non-existent-key");
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(missing))
            .unwrap()
    }

    let app = Router::new().route("/test-missing", get(missing_key_handler));

    let request = Request::builder()
        .uri("/test-missing")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap();
    assert_eq!(body_str, "non-existent-key");
}
