use crate::models::_entities::{fees, processes, processes_fees};
use crate::models::processes_fees::CreateNewProcessFee;
use crate::views::processes as ProcessView;
use axum::debug_handler;
use axum::extract::{Json, State};
use axum::response::Response;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProcessFee {
    pub process_fee_pid: Uuid,
    pub process_pid: Uuid,
    pub fee_pid: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteProcessFee {
    pub process_fee_pid: Uuid,
}

/// Creates a new process fee
///
/// # Errors
///
/// When could not create process fee or DB query error
#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<CreateNewProcessFee>,
) -> Result<Response> {
    let response = processes_fees::Model::create(&ctx.db, req_body).await;

    let _process_fee = match response {
        Ok(process_fee) => process_fee,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create process fee",);
            return format::json(());
        }
    };

    let processes = processes::Model::find_all(&ctx.db).await?;
    let fees = fees::Model::find_all(&ctx.db).await?;
    let process_fees = processes_fees::Model::find_all(&ctx.db).await?;
    format::json(ProcessView::ProcessView::from_model(
        processes,
        &process_fees,
        &fees,
    ))
}

/// Updates a process fee
///
/// # Errors
///
/// When could not find process fee by the given pid or DB query error
#[debug_handler]
pub async fn update(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<UpdateProcessFee>,
) -> Result<Response> {
    let update_process_fee_params = CreateNewProcessFee {
        process_pid: req_body.process_pid,
        fee_pid: req_body.fee_pid,
    };
    let res =
        processes_fees::Model::update(&ctx.db, req_body.process_fee_pid, update_process_fee_params)
            .await;

    let _process_fee = match res {
        Ok(process_fee) => process_fee,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not update process fee",);
            return format::json(());
        }
    };

    let processes = processes::Model::find_all(&ctx.db).await?;
    let fees = fees::Model::find_all(&ctx.db).await?;
    let process_fees = processes_fees::Model::find_all(&ctx.db).await?;
    format::json(ProcessView::ProcessView::from_model(
        processes,
        &process_fees,
        &fees,
    ))
}

/// Deletes a process fee
///
/// # Errors
///
/// When could not find process fee by the given pid or DB query error
#[debug_handler]
pub async fn del(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<DeleteProcessFee>,
) -> Result<Response> {
    let res = processes_fees::Model::delete(&ctx.db, req_body.process_fee_pid).await;

    let _process_fee = match res {
        Ok(process_fee) => process_fee,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not delete process fee",);
            return format::json(());
        }
    };

    let processes = processes::Model::find_all(&ctx.db).await?;
    let fees = fees::Model::find_all(&ctx.db).await?;
    let process_fees = processes_fees::Model::find_all(&ctx.db).await?;
    format::json(ProcessView::ProcessView::from_model(
        processes,
        &process_fees,
        &fees,
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/process_fees")
        .add("/create", post(create_new))
        .add("/edit", put(update))
        .add("/delete", delete(del))
}
