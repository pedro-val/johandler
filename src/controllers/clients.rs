use crate::models::_entities::clients;
use crate::models::clients::CreateNewClient;
use crate::views::clients::ClientViewResponse;
use axum::debug_handler;
use axum::extract::State;
use axum::Json;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClient {
    pub pid: Uuid,
    pub name: String,
    pub contact: String,
    pub phone: String,
    pub phone2: Option<String>,
    pub email: String,
    pub partner_pid: Option<Uuid>,
}

#[debug_handler]
pub async fn get_all(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<ClientViewResponse>>> {
    let clients = clients::Model::find_all(&ctx.db).await?;
    let mut client_views = Vec::new();

    for client in clients {
        let client_view = ClientViewResponse::from_model(&ctx.db, client).await?;
        client_views.push(client_view);
    }

    Ok(Json(client_views))
}

#[debug_handler]
pub async fn create_new(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateNewClient>,
) -> Result<Json<ClientViewResponse>> {
    let client = clients::Model::create(&ctx.db, params).await?;
    let client_view = ClientViewResponse::from_model(&ctx.db, client).await?;
    Ok(Json(client_view))
}

#[debug_handler]
pub async fn update(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateClient>,
) -> Result<Json<ClientViewResponse>> {
    let to_update_client = CreateNewClient {
        name: params.name.clone(),
        contact: params.contact.clone(),
        phone: params.phone.clone(),
        phone2: params.phone2.clone(),
        email: params.email.clone(),
        partner_pid: params.partner_pid.clone(),
    };
    let client_updated = clients::Model::update(&ctx.db, params.pid, to_update_client).await?;
    let client_view = ClientViewResponse::from_model(&ctx.db, client_updated).await?;
    Ok(Json(client_view))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/clients")
        .add("/all", get(get_all))
        .add("/create", post(create_new))
        .add("/edit", put(update))
}
