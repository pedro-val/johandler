use crate::models::_entities::partners;
use crate::models::partners::CreateNewPartner;
use crate::views::partners as PartnersView;
use axum::debug_handler;
use axum::extract::{Json, State};
use axum::response::Response;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditPartner {
    pub pid: Uuid,
    pub name: String,
    pub information: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

/// Creates a new partner
///
/// # Errors
///
/// When could not create partner or DB query error
#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<CreateNewPartner>,
) -> Result<Response> {
    let response = partners::Model::create(&ctx.db, req_body).await;

    let partner = match response {
        Ok(partner) => partner,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create partner",);
            return format::json(());
        }
    };

    format::json(PartnersView::PartnerView::from_model(partner))
}

/// Gets all partners
///
/// # Errors
///
/// When could not find partners or DB query error
#[debug_handler]
pub async fn get_all(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let partners = partners::Model::find_all(&ctx.db).await;

    let partners = match partners {
        Ok(partners) => partners,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not find partners",);
            return format::json(());
        }
    };

    format::json(PartnersView::PartnerView::from_model(partners))
}

/// Updates a partner
///
/// # Errors
///
/// When could not find partner by the given pid or DB query error
#[debug_handler]
pub async fn edit(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req_body): Json<EditPartner>,
) -> Result<Response> {
    let create_new_partner_params = CreateNewPartner {
        name: req_body.name.clone(),
        information: req_body.information.clone(),
        phone: req_body.phone.clone(),
        email: req_body.email.clone(),
    };
    let response = partners::Model::update(&ctx.db, req_body.pid, create_new_partner_params).await;

    let partner = match response {
        Ok(partner) => partner,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not update partner",);
            return format::json(());
        }
    };

    format::json(PartnersView::PartnerView::from_model(partner))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/partners")
        .add("/create", post(create_new))
        .add("/all", get(get_all))
        .add("/edit", put(edit))
}
