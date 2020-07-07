pub enum MarketType {
    Unknown = 0,
    AStocksChina = 0x0001,
    StocksHongKong = 0x0002,
    FuturesChina = 0x0010,

    StocksUSA = 0x0101,
}

impl MarketType {
    pub fn from_u32(value: u32) -> MarketType {
        match value {
            0x0001 => MarketType::AStocksChina,
            0x0002 => MarketType::StocksHongKong,
            0x0010 => MarketType::FuturesChina,
            0x0101 => MarketType::StocksUSA,
            _ => MarketType::Unknown,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            MarketType::AStocksChina => String::from("A股"),
            MarketType::AStocksChina => String::from("港股"),
            MarketType::FuturesChina => String::from("中国期货市场"),
            MarketType::StocksUSA => String::from("美股"),
            _ => String::from("未知市场"),
        }
    }
}