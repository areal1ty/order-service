use crate::handlers::order::Order;
use crate::state::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::{Postgres, Row, Transaction};
use std::sync::Arc;

pub async fn has_orders(
    state: &Arc<Arc<AppState>>
) -> Result<bool, StatusCode> {
    let queries = vec![
        "SELECT COUNT(*) FROM orders",
        "SELECT COUNT(*) FROM delivery",
        "SELECT COUNT(*) FROM payment",
        "SELECT COUNT(*) FROM items",
    ];

    for query in queries {
        let result = sqlx::query(query)
            .fetch_one(&state.db_pool)
            .await;

        if let Ok(row) = result {
            let count: i64 = row.try_get(0).unwrap_or(0);

            if count > 0 {
                return Ok(true);
            }
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(false)
}

pub async fn save_orders<'a, 'b>(
    mut transaction: &'a Transaction<Postgres>,
    order_data: &'b Order
) {
    sqlx::query(
        "INSERT INTO orders (order_uid, track_number, entry, locale, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(&order_data.order_uid)
        .bind(&order_data.track_number)
        .bind(&order_data.entry)
        .bind(&order_data.locale)
        .bind(&order_data.customer_id)
        .bind(&order_data.delivery_service)
        .bind(&order_data.shardkey)
        .bind(&order_data.sm_id)
        .bind(&order_data.date_created)
        .bind(&order_data.oof_shard)
        .execute(&mut transaction)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
}

pub async fn save_delivery(
    mut transaction: &Transaction<Postgres>,
    order_data: &Order
) {
    sqlx::query(
        "INSERT INTO delivery (order_uid, name, phone, zip, city, address, region, email)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&order_data.order_uid)
        .bind(&order_data.delivery.name)
        .bind(&order_data.delivery.phone)
        .bind(&order_data.delivery.zip)
        .bind(&order_data.delivery.city)
        .bind(&order_data.delivery.address)
        .bind(&order_data.delivery.region)
        .bind(&order_data.delivery.email)
        .execute(&mut transaction)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
}

pub async fn save_payment(
    mut transaction: &Transaction<Postgres>,
    order_data: &Order
) {
    sqlx::query(
        "INSERT INTO payment (order_uid, transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
        .bind(&order_data.order_uid)
        .bind(&order_data.payment.transaction)
        .bind(&order_data.payment.request_id)
        .bind(&order_data.payment.currency)
        .bind(&order_data.payment.provider)
        .bind(&order_data.payment.amount)
        .bind(&order_data.payment.payment_dt)
        .bind(&order_data.payment.bank)
        .bind(&order_data.payment.delivery_cost)
        .bind(&order_data.payment.goods_total)
        .bind(&order_data.payment.custom_fee)
        .execute(&mut transaction)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
}

pub async fn save_items(
    mut transaction: &Transaction<Postgres>,
    order_data: &Order
) {
    for item in &order_data.items {
        sqlx::query(
            "INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)")
            .bind(&order_data.order_uid)
            .bind(&item.chrt_id)
            .bind(&item.track_number)
            .bind(&item.price)
            .bind(&item.rid)
            .bind(&item.name)
            .bind(&item.sale)
            .bind(&item.size)
            .bind(&item.total_price)
            .bind(&item.nm_id)
            .bind(&item.brand)
            .bind(&item.status)
            .execute(&mut transaction)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
    }
}

pub async fn get_order_from_db(
    order_uid: &str,
    state: &State<Arc<AppState>>
) {
    let row = sqlx::query(
        r#"
        SELECT
            o.order_uid, o.track_number, o.entry, o.locale, o.internal_signature,
            o.customer_id, o.delivery_service, o.shardkey, o.sm_id, o.date_created, o.oof_shard,
            d.name as delivery_name, d.phone as delivery_phone, d.zip as delivery_zip,
            d.city as delivery_city, d.address as delivery_address, d.region as delivery_region, d.email as delivery_email,
            p.transaction as payment_transaction, p.request_id as payment_request_id, p.currency as payment_currency,
            p.provider as payment_provider, p.amount as payment_amount, p.payment_dt as payment_payment_dt,
            p.bank as payment_bank, p.delivery_cost as payment_delivery_cost, p.goods_total as payment_goods_total,
            p.custom_fee as payment_custom_fee,
            i.chrt_id as item_chrt_id, i.track_number as item_track_number, i.price as item_price,
            i.rid as item_rid, i.name as item_name, i.sale as item_sale, i.size as item_size,
            i.total_price as item_total_price, i.nm_id as item_nm_id, i.brand as item_brand, i.status as item_status
        FROM orders o
        JOIN delivery d ON o.order_uid = d.order_uid
        JOIN payment p ON o.order_uid = p.order_uid
        LEFT JOIN items i ON o.order_uid = i.order_uid
        WHERE o.order_uid = $1
        "#)
        .bind(order_uid)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    if row.is_empty() {
        Err(StatusCode::NOT_FOUND).unwrap();
    }
}

