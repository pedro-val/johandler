use crate::models::_entities::fees;
use crate::models::fees::CreateNewFee;
use crate::views::fees as FeesView;
use axum::debug_handler;
use axum::{extract::State, Json};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFee {
    pub pid: Uuid,
    pub fee: String,
    pub r#type: Option<String>,
}

/// Creates a new fee
///
/// # Errors
///
/// When could not create fee or DB query error
#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<CreateNewFee>,
) -> Result<Response> {
    let response = fees::Model::create(&ctx.db, req_body).await;

    let fee = match response {
        Ok(fee) => fee,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create fee",);
            return format::json(());
        }
    };

    format::json(FeesView::FeeView::from_model(fee))
}

/// Gets all fees
///
/// # Errors
///
/// When could not find fees or DB query error
#[debug_handler]
pub async fn get_all(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let fees = fees::Model::find_all(&ctx.db).await;

    let fees = match fees {
        Ok(fees) => fees,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not find fees",);
            return format::json(());
        }
    };

    format::json(FeesView::FeeView::from_model(fees))
}

/// Updates a fee
///
/// # Errors
///
/// When could not update fee or DB query error
#[debug_handler]
pub async fn edit(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<UpdateFee>,
) -> Result<Response> {
    let update_fee_params = CreateNewFee {
        fee: req_body.fee.clone(),
        r#type: req_body.r#type.clone(),
    };
    let response = fees::Model::update(&ctx.db, req_body.pid, update_fee_params).await;

    let fee = match response {
        Ok(fee) => fee,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not update fee",);
            return format::json(());
        }
    };

    format::json(FeesView::FeeView::from_model(fee))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/fees")
        .add("/create", post(create_new))
        .add("/all", get(get_all))
        .add("/edit", put(edit))
}
