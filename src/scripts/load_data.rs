use std::sync::Arc;
use axum::Json;
use crate::handlers::order::{create_order, Delivery, Item, Order, Payment};
use crate::state::state::AppState;

pub async fn load_data(state: &Arc<AppState>) {

        for i in 0..1000 {
            let new_order = generate_order(i);

            match create_order(Json(new_order), Arc::clone(&state)).await {
                Ok(status) => {
                    println!("Order created with status: {:?}", status);
                }
                Err(err) => {
                    eprintln!("Failed to create order: {:?}", err);
                }
            }
        }
    }

    fn generate_order(i: usize) -> Order {
        Order {
            order_uid: format!("order_uid_{}", i),
            track_number: format!("TRACK{}", i),
            entry: "WBIL".to_string(),
            delivery: Delivery {
                name: format!("Test User {}", i),
                phone: format!("+972000000{}", i),
                zip: "2639809".to_string(),
                city: "City".to_string(),
                address: format!("Street {}, House {}", i, i),
                region: "Region".to_string(),
                email: format!("user{}@mail.com", i),
            },
            payment: Payment {
                transaction: format!("transaction_{}", i),
                request_id: "".to_string(),
                currency: "USD".to_string(),
                provider: "wbpay".to_string(),
                amount: 1817,
                payment_dt: 1637907727,
                bank: "alpha".to_string(),
                delivery_cost: 1500,
                goods_total: 317,
                custom_fee: 0,
            },
            items: vec![
                Item {
                    chrt_id: 9934930,
                    track_number: format!("TRACK{}", i),
                    price: 453,
                    rid: format!("rid_{}", i),
                    name: "Mascaras".to_string(),
                    sale: 30,
                    size: "0".to_string(),
                    total_price: 317,
                    nm_id: 2389212,
                    brand: "Vivienne Sabo".to_string(),
                    status: 202,
                },
            ],
            locale: "en".to_string(),
            internal_signature: "".to_string(),
            customer_id: format!("customer_id_{}", i),
            delivery_service: "meest".to_string(),
            shardkey: "9".to_string(),
            sm_id: 99,
            date_created: "2021-11-26T06:22:19Z".to_string(),
            oof_shard: "1".to_string(),
        }
}
