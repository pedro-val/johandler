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

#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<CreateNewPartner>,
) -> Result<Response> {
    let create_new_partner_params = CreateNewPartner {
        name: req.name.clone(),
        information: req.information.clone(),
        phone: req.phone.clone(),
        email: req.email.clone(),
    };
    let res = partners::Model::create(&ctx.db, create_new_partner_params).await;

    let partner = match res {
        Ok(partner) => partner,
        Err(err) => {
            tracing::info!(message = err.to_string(), "could not create partner",);
            return format::json(());
        }
    };

    format::json(PartnersView::PartnerView::from_model(partner))
}

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

#[debug_handler]
pub async fn edit(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(req): Json<EditPartner>,
) -> Result<Response> {
    let create_new_partner_params = CreateNewPartner {
        name: req.name.clone(),
        information: req.information.clone(),
        phone: req.phone.clone(),
        email: req.email.clone(),
    };
    let res = partners::Model::update(&ctx.db, req.pid, create_new_partner_params).await;

    let partner = match res {
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
