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

/// Creates a new seller
///
/// # Errors
///
/// When could not create seller or DB query error
#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(edit_request): Json<CreateNewSeller>,
) -> Result<Response> {
    let res = sellers::Model::create(&ctx.db, edit_request).await;

    let seller = match res {
        Ok(seller) => seller,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create seller",);
            return format::json(());
        }
    };

    format::json(SellersView::SellerView::from_model(seller))
}

/// Gets all sellers
///     
/// # Errors
///
/// When could not find sellers or DB query error
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

/// Edits a seller
///
/// # Errors
///
/// When could not find seller by the given pid or DB query error
#[debug_handler]
pub async fn edit(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(edit_request): Json<EditSellet>,
) -> Result<Response> {
    let create_new_seller_params = CreateNewSeller {
        name: edit_request.name.clone(),
    };
    let response =
        sellers::Model::update(&ctx.db, edit_request.pid, create_new_seller_params).await;

    let seller = match response {
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
