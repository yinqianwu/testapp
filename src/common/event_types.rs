use diesel::sql_types::BigInt;

pub enum EventType {
    Unknown = 0,

    /* 公司事件 */
    Corporate = 0x01000000,

    /* 人事相关事件 */
    Personnel = 0x01010000,
    // 高层变动
    HLChange = 0x01010100,
    // 人身安全
    PersonalSafety = 0x01010200,
    // 婚姻危机
    MarriageCrisis = 0x01010300,
    // 高层违法
    HLLawViolation = 0x01010400,


    /* 公司财务事件 */
    CorporateFinance = 0x01020000,
    // 业绩预告
    PerfForecast = 0x01020100,
    // 业绩预增
    PerfForecastRise = 0x01020101,
    // 业绩预减
    PerfForecastFall = 0x01020102,

    // 产品销售火爆
    ProductSellWell = 0x01020301,

    /* 股权资产变动事件 */
    CapitalsChange = 0x01030000,
    // 资产重组
    Recapital  = 0x01030100,
    // 转让子公司
    SubsidiaryTransfer = 0x01030101,


    /* 二级市场事件 */
    SecondaryMarket = 0x01040000,

    // 资金流入
    CapitalInflows = 0x01040101,

    // 市值增加
    MarketValueRise = 0x01040103,


    /* 融资交易事件 */
    MarginTrading = 0x01050000,

    // 发行上市
    IPORelated = 0x01050100,

    // 新股申购
    NewShares = 0x01050101,



    /* 宏观事件 */
    Macro = 0x02000000,
}

impl EventType {
    pub fn from_u32(value: u32) -> EventType {
        match value {
            0x01020301 => EventType::ProductSellWell,
            0x01030100 => EventType::SubsidiaryTransfer,
            0x01040101 => EventType::CapitalInflows,
            0x01040103 => EventType::MarketValueRise,
            0x01050101 => EventType::NewShares,
            _ => EventType::Unknown,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            EventType::ProductSellWell => String::from("产品销售火爆"),
            EventType::SubsidiaryTransfer => String::from("转让子公司"),
            EventType::CapitalInflows => String::from("资金流入"),
            EventType::MarketValueRise => String::from("市值增加"),
            EventType::NewShares => String::from("新股申购"),
            _ => String::from("未知标签"),
        }
    }
    pub fn vi32_to_vstring(event_values: Vec<i32>) -> Vec<String> {

        let mut v: Vec<String> = Vec::new();
        for event_value in event_values {
            let event_type = EventType::from_u32(event_value as u32);
            match event_type {
                EventType::Unknown => (),
                _ => v.push(event_type.to_string()),
            }
        }
        v
    }
}



