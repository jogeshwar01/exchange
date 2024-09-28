use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Asset {
    USDT,
    BTC,
    ETH,
    SOL,
}

impl Asset {
    pub fn from_str(asset_str: &str) -> Result<Asset, &'static str> {
        // static lifetime because Err str slice is static
        match asset_str {
            "USDT" => Ok(Asset::USDT),
            "BTC" => Ok(Asset::BTC),
            "ETH" => Ok(Asset::ETH),
            "SOL" => Ok(Asset::SOL),
            _ => Err("Unsupported asset"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetPair {
    pub base: Asset,
    pub quote: Asset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    BUY,
    SELL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    LIMIT,
    MARKET,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Filled,
    PartiallyFilled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub price: Decimal,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub order_id: String,
    pub user_id: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub order_status: OrderStatus,
    pub timestamp: i64, // chrono::Utc::now().timestamp_millis();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub price: Decimal,
    pub quantity: Decimal,
    pub trade_id: u64,
    pub other_user_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOrderResult {
    pub executed_quantity: Decimal,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    pub market: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrder {
    pub order_id: String,
    pub user_id: String,
    pub price: Decimal,
    pub side: OrderSide,
    pub market: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenOrders {
    pub user_id: String,
    pub market: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderRequests {
    CreateOrder(CreateOrder),
    CancelOrder(CancelOrder),
    GetOpenOrders(GetOpenOrders),
}
