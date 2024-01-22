use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};
use std::env;

pub async fn validate_authorization(req: Request, next: Next) -> Response<Body> {
    let auth_header = req.headers().get("authorization");

    match auth_header {
        Some(authorization) => {
            let bearer_token = env::var("BEARER_TOKEN").expect("BEARER TOKEN not set");
            if bearer_token == authorization.to_str().unwrap().to_string() {
                return next.run(req).await;
            }

            return unauthorize_response();
        }
        None => unauthorize_response(),
    }
}

fn unauthorize_response() -> Response<Body> {
    let response = Response::new(Body::default());
    let (mut parts, body) = response.into_parts();
    parts.status = StatusCode::UNAUTHORIZED;
    let unauthorize_response = Response::from_parts(parts, body);

    unauthorize_response
}
