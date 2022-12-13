use crate::data::currency::Currency;
use crate::data::item::Item;
use crate::data::price::Price;
use crate::io::prices::checker::ItemPriceChecker;
use crate::io::prices::payloads::poeprices::PoePricesPayload;
use async_trait::async_trait;

pub struct PoePricesItemPriceChecker {}

impl PoePricesItemPriceChecker {
    async fn get_price_payload(
        item: Item,
        league: String,
    ) -> Result<PoePricesPayload, reqwest::Error> {
        let params = [
            ("i", item.as_base64()),
            ("l", league),
            ("s", "awakened-cli-trade".to_string()),
        ];
        let url =
            reqwest::Url::parse_with_params("https://www.poeprices.info/api", &params).unwrap();
        reqwest::get(url).await.unwrap().json().await
    }
}

#[async_trait]
impl ItemPriceChecker for PoePricesItemPriceChecker {
    async fn check_item_price(item: Item, league: String) -> Option<Price> {
        match PoePricesItemPriceChecker::get_price_payload(item, league).await {
            Ok(payload) => Some(Price {
                currency: Currency::Chaos,
                min: payload.min,
                max: payload.max,
                confidence: payload.pred_confidence_score,
            }),
            Err(e) => {
                println!("Request to fetch item price failed. {}", e);
                None
            }
        }
    }
}
