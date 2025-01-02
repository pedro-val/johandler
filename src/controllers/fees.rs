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

#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateNewFee>,
) -> Result<Response> {
    let create_new_fee_params = CreateNewFee {
        fee: req.fee.clone(),
        r#type: req.r#type.clone(),
    };
    let res = fees::Model::create(&ctx.db, create_new_fee_params).await;

    let fee = match res {
        Ok(fee) => fee,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create fee",);
            return format::json(());
        }
    };

    format::json(FeesView::FeeView::from_model(fee))
}

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

#[debug_handler]
pub async fn edit(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<UpdateFee>,
) -> Result<Response> {
    let update_fee_params = CreateNewFee {
        fee: req.fee.clone(),
        r#type: req.r#type.clone(),
    };
    let res = fees::Model::update(&ctx.db, req.pid, update_fee_params).await;

    let fee = match res {
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
