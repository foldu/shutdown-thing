use crate::{
    control::{PowerController, PowerState},
    models::{PowerRequest, PowerResponse, StatusResponse, VersionResponse},
};
use axum::{extract::State, Json};
use std::{sync::Arc, time::Duration};

pub async fn version() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn status() -> Json<StatusResponse> {
    Json(StatusResponse { status: true })
}

fn control(
    controller: &Arc<PowerController>,
    req: &PowerRequest,
    state: PowerState,
) -> Json<PowerResponse> {
    let timeout = req.timeout.unwrap_or(0);
    tracing::info!(state = ?state, timeout, "Received request to change state");
    let timeout = Duration::from_secs(u64::from(timeout));
    match controller.change_state(timeout, state) {
        Ok(()) => Json(PowerResponse::default()),
        Err(e) => Json(PowerResponse {
            status: false,
            error: Some(format!("Failed to change state: {}", e)),
        }),
    }
}

#[tracing::instrument(skip(controller))]
pub async fn shutdown(
    controller: State<Arc<PowerController>>,
    req: Json<PowerRequest>,
) -> Json<PowerResponse> {
    control(&controller, &req, PowerState::Poweroff)
}

#[tracing::instrument(skip(controller))]
pub async fn reboot(
    controller: State<Arc<PowerController>>,
    req: Json<PowerRequest>,
) -> Json<PowerResponse> {
    control(&controller, &req, PowerState::Reboot)
}

#[tracing::instrument(skip(controller))]
pub async fn sleep(
    controller: State<Arc<PowerController>>,
    req: Json<PowerRequest>,
) -> Json<PowerResponse> {
    control(&controller, &req, PowerState::Sleep)
}
