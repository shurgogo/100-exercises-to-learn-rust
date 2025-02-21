use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use outro_08::{
    data::{TicketDraft, TicketId},
    store::TicketStore,
};

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(TicketStore::new()));

    let app = Router::new()
        .route("/v1/tickets", get(list).post(create))
        .route("/v1/tickets/{id}", patch(update))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list(
    State(store): State<Arc<Mutex<TicketStore>>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let store = store.lock().unwrap();
    match store.get_all() {
        Ok(tickets) => Ok(Json(tickets)),
        Err(error) => Err((StatusCode::FORBIDDEN, format!("{:?}", error))),
    }
}

async fn create(
    State(store): State<Arc<Mutex<TicketStore>>>,
    Json(ticket_draft): Json<TicketDraft>,
) -> impl IntoResponse {
    let mut store = store.lock().unwrap();
    let ticket = store.add_ticket(ticket_draft).unwrap();
    (StatusCode::CREATED, Json(ticket))
}

async fn update(
    Path(id): Path<TicketId>,
    State(store): State<Arc<Mutex<TicketStore>>>,
    Json(ticket_draft): Json<TicketDraft>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut store = store.lock().unwrap();
    let id = store.update(id, ticket_draft).unwrap();
    Ok(Json(id))
}
