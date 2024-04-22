use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use rand::Rng;
use serde_json::json;

use crate::{model::UrlModel, schema::CreateUrlSchema, AppState};

pub async fn create_url_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUrlSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id: String = uuid::Uuid::new_v4().to_string();
    let id_clone = id.clone();
    let code = body.code.unwrap_or_else(|| generate_random_code());
    let query_result = sqlx::query(r#"INSERT INTO urls (id, code, url) VALUES (?, ?, ?, ?)"#)
        .bind(id)
        .bind(code.clone())
        .bind(body.url.clone())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "URL with this code already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let response = serde_json::json!({
        "status": "success",
        "data": {
            "id": id_clone,
            "url": body.url,
            "code": code.clone(),
            "created_at": chrono::Utc::now(),
            "updated_at": chrono::Utc::now(),
        },
    });

    Ok(Json(response))
}

fn generate_random_code() -> String {
    let code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    code
}

pub async fn get_url_handler(
    Path(code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as::<_, UrlModel>(r#"SELECT * FROM url WHERE url = ?"#)
        .bind(&code)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(data) => {
            let redirect = Redirect::to(data.url.as_str());
            return Ok(redirect.into_response());
        }
        Err(sqlx::Error::RowNotFound) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"status": "error","message": "URL with this code not found"})),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };
}
