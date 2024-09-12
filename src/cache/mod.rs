pub mod util;

use std::sync::Arc;
use dashmap::DashMap;
use crate::handlers::order::Order;
/*
/// Тип для кэша заказов
pub type OrderCache = Arc<DashMap<String, Order>>;

/// Инициализация кэша
pub fn init_cache() -> OrderCache {
    Arc::new(DashMap::new())
}

/// Добавление заказа в кэш
pub fn add_order_to_cache(cache: &OrderCache, order: Order) {
    cache.insert(order.order_uid.clone(), order);
}

/// Получение заказа из кэша по его UID
pub fn get_order_from_cache(cache: &OrderCache, order_uid: &str) -> Option<Order> {
    cache.get(order_uid).map(|entry| entry.clone())
}

/// Удаление заказа из кэша по его UID
pub fn remove_order_from_cache(cache: &OrderCache, order_uid: &str) {
    cache.remove(order_uid);

 */
