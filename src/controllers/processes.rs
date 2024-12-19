use crate::models::_entities::processes;
use crate::models::processes::CreateNewProcess;
use crate::views::processes as ProcessesView;
use axum::debug_handler;
use axum::extract::{Json, State};
use axum::response::Response;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateNewProcess>,
) -> Result<Response> {
    let create_new_process_params = CreateNewProcess {
        case_type: req.case_type.clone(),
    };
    let res = processes::Model::create(&ctx.db, create_new_process_params).await;

    let process = match res {
        Ok(process) => process,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create process",);
            return format::json(());
        }
    };

    format::json(ProcessesView::ProcessView::from_model(process))
}

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

    format::json(ProcessesView::ProcessView::from_model(processes))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/processes")
        .add("/create", post(create_new))
        .add("/all", get(get_all))
}
