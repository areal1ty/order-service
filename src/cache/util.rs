use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use redis::Commands;
use crate::handlers::order::Order;
use crate::state::state::AppState;

pub async fn has_orders_in_cache(
    order_uid: &str,
    State(state): &State<Arc<AppState>>,
) -> Result<Option<Order>, StatusCode> {
    let mut redis_conn = state.redis_connection.clone();
    if let Ok(cached_order) = redis_conn.get::<_, String>(order_uid).await {
        if let Ok(order) = serde_json::from_str::<Order>(&cached_order) {
            return Ok(Some(order));
        }
    }
    Ok(None)
}

pub async fn cache_order(
    order_uid: &str,
    order: &Order,
    State(state): State<Arc<AppState>>,
) -> Result<(), StatusCode> {
    let mut redis_conn = state.redis_connection.clone();
    let serialized_order = serde_json::to_string(&order).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let _: () = redis_conn
        .set_ex(order_uid, serialized_order, 3600)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}