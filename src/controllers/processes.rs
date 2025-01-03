use crate::models::_entities::{fees, processes, processes_fees};
use crate::models::processes::CreateNewProcess;
use crate::views::processes as ProcessesView;
use axum::debug_handler;
use axum::extract::{Json, State};
use axum::response::Response;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProcess {
    pub pid: Uuid,
    pub case_type: String,
}

/// Creates a new process
///
/// # Errors
///
/// When could not create process or DB query error
#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<CreateNewProcess>,
) -> Result<Response> {
    let response = processes::Model::create(&ctx.db, req_body).await;

    let process = match response {
        Ok(process) => process,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create process",);
            return format::json(());
        }
    };

    let fees = fees::Model::find_all(&ctx.db).await?;
    let process_fees = processes_fees::Model::find_all(&ctx.db).await?;
    format::json(ProcessesView::ProcessView::from_model(
        process,
        &process_fees,
        &fees,
    ))
}

/// Gets all processes
///
/// # Errors
///
/// When could not find processes or DB query error
#[debug_handler]
pub async fn get_all(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let processes = processes::Model::find_all(&ctx.db).await;

    let processes = match processes {
        Ok(processes) => processes,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not find processes",);
            return format::json(());
        }
    };
    let fees = fees::Model::find_all(&ctx.db).await?;
    let process_fees = processes_fees::Model::find_all(&ctx.db).await?;
    format::json(ProcessesView::ProcessView::from_model(
        processes,
        &process_fees,
        &fees,
    ))
}

/// Updates a process
///
/// # Errors
///
/// When could not find process by the given pid or DB query error
#[debug_handler]
pub async fn update(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<UpdateProcess>,
) -> Result<Response> {
    let update_process_params = CreateNewProcess {
        case_type: req_body.case_type.clone(),
    };
    let response = processes::Model::update(&ctx.db, req_body.pid, update_process_params).await;

    let process = match response {
        Ok(process) => process,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not update process",);
            return format::json(());
        }
    };

    let fees = fees::Model::find_all(&ctx.db).await?;
    let process_fees = processes_fees::Model::find_all(&ctx.db).await?;
    format::json(ProcessesView::ProcessView::from_model(
        process,
        &process_fees,
        &fees,
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/processes")
        .add("/create", post(create_new))
        .add("/all", get(get_all))
        .add("/edit", put(update))
}
