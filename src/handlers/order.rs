use crate::db::util::{get_order_from_db, save_delivery, save_items, save_orders, save_payment};
use axum::{http::StatusCode, Json};
use sqlx::{Postgres, Transaction};
use std::sync::Arc;
use axum::extract::State;
use crate::cache::util::{cache_order, has_orders_in_cache};
use crate::state::state::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub order_uid: String,
    pub track_number: String,
    pub entry: String,
    pub delivery: Delivery,
    pub payment: Payment,
    pub items: Vec<Item>,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: u32,
    pub date_created: String,
    pub oof_shard: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Delivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Payment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: u32,
    pub payment_dt: u64,
    pub bank: String,
    pub delivery_cost: u32,
    pub goods_total: u32,
    pub custom_fee: u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub chrt_id: u32,
    pub track_number: String,
    pub price: u32,
    pub rid: String,
    pub name: String,
    pub sale: u32,
    pub size: String,
    pub total_price: u32,
    pub nm_id: u32,
    pub brand: String,
    pub status: u32,
}

pub async fn get_order(
    app_state: &State<Arc<AppState>>,
    order_uid: &str,
) -> Result<Order, StatusCode> {
    has_orders_in_cache(order_uid, app_state).await?;
    let row = get_order_from_db(order_uid, app_state);
    let mut order = Order {
        order_uid: row[0].order_uid.clone(),
        track_number: row[0].track_number.clone(),
        entry: row[0].entry.clone(),
        delivery: Delivery {
            name: row[0].delivery_name.clone(),
            phone: row[0].delivery_phone.clone(),
            zip: row[0].delivery_zip.clone(),
            city: row[0].delivery_city.clone(),
            address: row[0].delivery_address.clone(),
            region: row[0].delivery_region.clone(),
            email: row[0].delivery_email.clone(),
        },
        payment: Payment {
            transaction: row[0].payment_transaction.clone(),
            request_id: row[0].payment_request_id.clone(),
            currency: row[0].payment_currency.clone(),
            provider: row[0].payment_provider.clone(),
            amount: row[0].payment_amount as u32,
            payment_dt: row[0].payment_payment_dt as u64,
            bank: row[0].payment_bank.clone(),
            delivery_cost: row[0].payment_delivery_cost as u32,
            goods_total: row[0].payment_goods_total as u32,
            custom_fee: row[0].payment_custom_fee as u32,
        },
        items: vec![],
        locale: row[0].locale.clone(),
        internal_signature: row[0].internal_signature.clone(),
        customer_id: row[0].customer_id.clone(),
        delivery_service: row[0].delivery_service.clone(),
        shardkey: row[0].shardkey.clone(),
        sm_id: row[0].sm_id as u32,
        date_created: row[0].date_created.clone(),
        oof_shard: row[0].oof_shard.clone(),
    };

    for item_row in row {
        if let Some(_item_id) = item_row.item_chrt_id {
            order.items.push(Item {
                chrt_id: item_row.item_chrt_id.unwrap_or_default() as u32,
                track_number: item_row.item_track_number.clone().unwrap_or_default(),
                price: item_row.item_price.unwrap_or_default() as u32,
                rid: item_row.item_rid.clone().unwrap_or_default(),
                name: item_row.item_name.clone().unwrap_or_default(),
                sale: item_row.item_sale.unwrap_or_default() as u32,
                size: item_row.item_size.clone().unwrap_or_default(),
                total_price: item_row.item_total_price.unwrap_or_default() as u32,
                nm_id: item_row.item_nm_id.unwrap_or_default() as u32,
                brand: item_row.item_brand.clone().unwrap_or_default(),
                status: item_row.item_status.unwrap_or_default() as u32,
            });
        }
    }

    cache_order(order_uid, &app_state, &order).await?;
    Ok(order)
}

pub async fn create_order(
    Json(order_data): Json<Order>,
    state: Arc<AppState>
) -> Result<StatusCode, StatusCode> {
    let mut transaction: Transaction<Postgres> = state.db_pool.begin().await.unwrap();

    save_orders(&transaction, &order_data).await;
    save_delivery(&transaction, &order_data).await;
    save_payment(&transaction, &order_data).await;
    save_items(&transaction, &order_data).await;

    transaction.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED)
}