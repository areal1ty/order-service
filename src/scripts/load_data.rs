use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;
use rand::Rng;
use chrono::Utc;
use std::time::SystemTime;

pub async fn load_data() -> Result<(), sqlx::Error> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/database")
        .await?;

    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let order_uid = Uuid::new_v4();
        let track_number = format!("TRACK{}", rng.gen_range(1000000..9999999));
        let entry = "web";
        let locale = "en";
        let customer_id = format!("CUSTOMER{}", rng.gen_range(1000..9999));
        let delivery_service = "DHL";
        let shardkey = "abc";
        let sm_id = rng.gen_range(1..100);
        let date_created = Utc::now().naive_utc();
        let oof_shard = "xyz";

        // Вставка данных в таблицу orders
        sqlx::query!(
            r#"
            INSERT INTO orders (order_uid, track_number, entry, locale, customer_id,
            delivery_service, shardkey, sm_id, date_created, oof_shard)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            order_uid, track_number, entry, locale, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
        )
            .execute(&pool)
            .await?;

        // Вставка данных в таблицу delivery
        let delivery_name = "John Doe";
        let delivery_phone = format!("+1{}", rng.gen_range(1000000000..9999999999));
        let delivery_zip = format!("{}", rng.gen_range(10000..99999));
        let delivery_city = "New York";
        let delivery_address = "123 Main St";
        let delivery_region = "NY";
        let delivery_email = "john@example.com";

        sqlx::query!(
            r#"
            INSERT INTO delivery (order_uid, name, phone, zip, city, address, region, email)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            order_uid, delivery_name, delivery_phone, delivery_zip, delivery_city, delivery_address, delivery_region, delivery_email
        )
            .execute(&pool)
            .await?;

        // Вставка данных в таблицу payment
        let payment_transaction = format!("TXN{}", rng.gen_range(1000000..9999999));
        let payment_request_id = format!("REQ{}", rng.gen_range(100000..999999));
        let payment_currency = "USD";
        let payment_provider = "Visa";
        let payment_amount = rng.gen_range(100..1000);
        let payment_dt = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        let payment_bank = "Chase";
        let delivery_cost = rng.gen_range(10..100);
        let goods_total = rng.gen_range(100..1000);
        let custom_fee = rng.gen_range(5..50);

        sqlx::query!(
            r#"
            INSERT INTO payment (order_uid, transaction, request_id, currency, provider,
            amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            order_uid, payment_transaction, payment_request_id, payment_currency, payment_provider,
            payment_amount, payment_dt, payment_bank, delivery_cost, goods_total, custom_fee
        )
            .execute(&pool)
            .await?;

        // Вставка данных в таблицу items
        for _ in 0..rng.gen_range(1..5) {
            let chrt_id = rng.gen_range(100000..999999);
            let item_track_number = format!("ITRACK{}", rng.gen_range(1000000..9999999));
            let price = rng.gen_range(50..500);
            let rid = format!("RID{}", rng.gen_range(100000..999999));
            let name = format!("Item{}", rng.gen_range(1..100));
            let sale = rng.gen_range(0..30);
            let size = "M";
            let total_price = price - (price * sale / 100);
            let nm_id = rng.gen_range(1000000..9999999);
            let brand = "SomeBrand";
            let status = 0;

            sqlx::query!(
                r#"
                INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                "#,
                order_uid, chrt_id, item_track_number, price, rid, name, sale, size, total_price, nm_id, brand, status
            )
                .execute(&pool)
                .await?;
        }
    }

    println!("Inserted 1000 orders with associated data.");
    Ok(())
}
