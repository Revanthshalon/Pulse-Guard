use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

use crate::errors::AccessGridResult;

pub async fn get_service_health() -> AccessGridResult<(StatusCode, Json<Value>)> {
    let response_status = StatusCode::OK;
    let response_body = Json(json!({
        "status": "Access Grid Service is running",
    }));
    Ok((response_status, response_body))
}
