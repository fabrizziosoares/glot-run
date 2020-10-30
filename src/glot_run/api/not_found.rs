use crate::glot_run::config;
use crate::glot_run::api;



pub fn handle(_: &config::Config, _: &mut tiny_http::Request) -> Result<Vec<u8>, api::ErrorResponse> {

    Err(api::ErrorResponse{
        status_code: 404,
        body: serde_json::to_vec_pretty(&api::ErrorBody{
            error: "route.not_found".to_string(),
            message: "Route not found".to_string(),
        }).unwrap_or_else(|_| "Route not found".to_string().as_bytes().to_vec())
    })
}
