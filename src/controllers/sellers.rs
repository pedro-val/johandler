use crate::models::_entities::sellers;
use crate::models::sellers::CreateNewSeller;
use crate::views::sellers as SellersView;
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditSellet {
    pub pid: Uuid,
    pub name: String,
}

#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateNewSeller>,
) -> Result<Response> {
    let create_new_seller_params = CreateNewSeller {
        name: req.name.clone(),
    };
    let res = sellers::Model::create(&ctx.db, create_new_seller_params).await;

    let seller = match res {
        Ok(seller) => seller,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create seller",);
            return format::json(());
        }
    };

    format::json(SellersView::SellerView::from_model(seller))
}

#[debug_handler]
pub async fn get_all(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let sellers = sellers::Model::find_all(&ctx.db).await;

    let sellers = match sellers {
        Ok(sellers) => sellers,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not find sellers",);
            return format::json(());
        }
    };

    format::json(SellersView::SellerView::from_model(sellers))
}

#[debug_handler]
pub async fn edit(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<EditSellet>,
) -> Result<Response> {
    let create_new_seller_params = CreateNewSeller {
        name: req.name.clone(),
    };
    let res = sellers::Model::update(&ctx.db, req.pid, create_new_seller_params).await;

    let seller = match res {
        Ok(seller) => seller,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create seller",);
            return format::json(());
        }
    };

    format::json(SellersView::SellerView::from_model(seller))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/sellers")
        .add("/create", post(create_new))
        .add("/all", get(get_all))
        .add("/edit", put(edit))
}
