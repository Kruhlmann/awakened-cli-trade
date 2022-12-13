use crate::data::item::Item;
use crate::data::price::Price;
use async_trait::async_trait;

#[async_trait]
pub trait ItemPriceChecker {
    async fn check_item_price(item: Item, league: String) -> Option<Price>;
}
