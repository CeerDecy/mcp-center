use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use mc_common::app::{AppState, Response};
use mc_db::model::SystemSettings;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetSettingsRequest {
    pub setting_name: Option<String>,
}

#[derive(Serialize)]
pub struct ListSettingsResponse {
    pub settings: Vec<SystemSettings>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateSettingItem {
    pub setting_name: String,
    pub setting_value: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateSettingsRequest {
    pub settings: Vec<UpdateSettingItem>,
}

#[derive(Serialize)]
pub struct UpdateSettingsResponse {
    pub settings: Vec<SystemSettings>,
}

pub async fn list_settings(
    State(state): State<AppState>,
    Query(request): Query<GetSettingsRequest>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let settings_handler = match &state.handlers().system_settings_handler {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't get system settings handler".to_string(),
            ));
        }
        Some(handler) => handler,
    };

    let settings = if let Some(setting_name) = request.setting_name {
        let setting = settings_handler
            .get_by_name(&setting_name)
            .await
            .map_err(|e| {
                tracing::error!("Failed to query system settings {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to query system settings".to_string(),
                )
            })?;

        let Some(setting) = setting else {
            return Err((StatusCode::NOT_FOUND, "Setting not found".to_string()));
        };

        vec![setting]
    } else {
        settings_handler.list_all().await.map_err(|e| {
            tracing::error!("Failed to list system settings {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list system settings".to_string(),
            )
        })?
    };

    let data = serde_json::to_value(ListSettingsResponse { settings }).map_err(|e| {
        tracing::error!("Failed to parse system settings {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    Ok(Json(Response::new(Some(data))))
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(request): Json<UpdateSettingsRequest>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let settings_handler = match &state.handlers().system_settings_handler {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't get system settings handler".to_string(),
            ));
        }
        Some(handler) => handler,
    };

    if request.settings.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "settings must not be empty".to_string(),
        ));
    }

    let mut updated = Vec::with_capacity(request.settings.len());
    for setting in &request.settings {
        let res = settings_handler
            .upsert(&setting.setting_name, &setting.setting_value)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update system settings {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update system settings".to_string(),
                )
            })?;
        updated.push(res);
    }

    let data = serde_json::to_value(UpdateSettingsResponse { settings: updated }).map_err(|e| {
        tracing::error!("Failed to parse system settings {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    Ok(Json(Response::new(Some(data))))
}
